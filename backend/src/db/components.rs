use crate::db::cycles::Cycle;
use crate::db_log::Logger;
use crate::Result;
use diesel::dsl::max;
use diesel::sql_types::{BigInt, Integer, Text};
use diesel::{
    sql_query, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug, Clone)]
pub struct Component {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub domain_id: i32,
    #[sql_type = "Integer"]
    pub rank: i32,
    #[sql_type = "Text"]
    pub title: String,
}

impl Component {
    pub fn by_domain_id(db: &PgConnection, domain_id: i32) -> Result<Vec<Component>> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        let components = dsl::components
            .order(components::rank)
            .filter(components::domain_id.eq(domain_id))
            .load::<Component>(db);
        Ok(components?)
    }

    pub fn by_id(db: &PgConnection, id: i32) -> Result<Component> {
        use crate::schema::components::dsl;
        let component = dsl::components.find(id).get_result::<Component>(db)?;
        Ok(component)
    }

    pub fn next_component_id(db: &PgConnection, domain_id: i32, rank: i32) -> Result<Option<i32>> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        let id = dsl::components
            .select(components::id)
            .order(components::rank)
            .filter(components::rank.gt(rank))
            .filter(components::domain_id.eq(domain_id))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    pub fn previous_component_id(
        db: &PgConnection,
        domain_id: i32,
        rank: i32,
    ) -> Result<Option<i32>> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        let id = dsl::components
            .select(components::id)
            .order(components::rank.desc())
            .filter(components::rank.lt(rank))
            .filter(components::domain_id.eq(domain_id))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    pub fn all(db: &PgConnection) -> Result<Vec<Component>> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        Ok(dsl::components
            .order(components::rank)
            .load::<Component>(db)?)
    }

    pub fn all_ordered(db: &PgConnection) -> Result<Vec<Component>> {
        Ok(sql_query(
            "
SELECT components.id, components.domain_id, components.rank, components.title
    FROM components
        LEFT JOIN domains
            ON domains.id = components.domain_id
    ORDER BY domains.rank, components.rank",
        )
        .get_results(db)?)
    }

    pub fn set_title(db: &PgConnection, log: &Logger, id: i32, title: String) -> Result<()> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        diesel::update(dsl::components.find(id))
            .set(components::title.eq(title.clone()))
            .execute(db)?;
        log.update1("components".to_string(), id, "title".to_string(), title)?;
        Ok(())
    }

    pub fn set_rank(db: &PgConnection, id: i32, rank: i32) -> Result<()> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        diesel::update(dsl::components.find(id))
            .set(components::rank.eq(rank))
            .execute(db)?;
        Ok(())
    }

    pub fn set_domain_id(db: &PgConnection, id: i32, domain_id: i32) -> Result<()> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        diesel::update(dsl::components.find(id))
            .set(components::domain_id.eq(domain_id))
            .execute(db)?;
        Ok(())
    }

    pub fn set_components_rank(db: &PgConnection, components: Vec<Component>) -> Result<()> {
        // components array arrives sorted as the user wants it
        // with invalid rank inside that I need to set
        let mut rank = 1;
        for component in components {
            Component::set_rank(db, component.id, rank)?;
            rank += 1;
        }
        Ok(())
    }

    pub fn max_rank(db: &PgConnection, domain_id: i32) -> Result<i32> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        // Select the current max rank
        let max_rank: Option<i32> = dsl::components
            .select(max(components::rank))
            .filter(components::domain_id.eq(domain_id))
            .first(db)?;
        Ok(max_rank.unwrap_or(0))
    }

    /// Return the new Component
    pub fn append(db: &PgConnection, domain_id: i32, title: String) -> Result<Component> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        // Compute the needed rank
        let rank: i32 = Component::max_rank(db, domain_id)? + 1;
        // insert
        let component_id: i32 = diesel::insert_into(dsl::components)
            .values((
                components::rank.eq(rank),
                components::domain_id.eq(domain_id),
                components::title.eq(title),
            ))
            .returning(components::id)
            .get_result(db)?;
        // return the new one
        Component::by_id(db, component_id)
    }

    pub fn delete(db: &PgConnection, id: i32) -> Result<()> {
        use crate::schema::components;
        use crate::schema::components::dsl;
        let component = Component::by_id(db, id)?;
        // Delete
        diesel::delete(dsl::components.filter(components::id.eq(id))).execute(db)?;
        // Fix rank of the domain_id
        Component::fix_rank(db, component.domain_id)?;
        Ok(())
    }

    pub fn move_to_domain(db: &PgConnection, component_id: i32, domain_id: i32) -> Result<()> {
        let component = Component::by_id(db, component_id)?;
        // Compute the needed rank in the next domain
        let rank: i32 = Component::max_rank(db, domain_id)? + 1;
        // Update
        Component::set_rank(db, component_id, rank)?;
        Component::set_domain_id(db, component_id, domain_id)?;
        // Fix rank of the previous domain_id
        Component::fix_rank(db, component.domain_id)?;
        Ok(())
    }

    pub fn fix_rank(db: &PgConnection, domain_id: i32) -> Result<()> {
        let components = Component::by_domain_id(db, domain_id)?;
        Component::set_components_rank(db, components)?;
        Ok(())
    }
}

#[derive(Serialize, Clone)]
pub struct ComponentSummary {
    pub id: i32,
    pub domain_id: i32,
    pub rank: i32,
    pub title: String,
    pub nb_competencies: i64,
    pub nb_evaluations: i64,
    pub nb_empty: i64,
    pub nb_empty_percent: f64,
    pub nb_in_progress: i64,
    pub nb_in_progress_percent: f64,
    pub nb_acquired: i64,
    pub nb_acquired_percent: f64,
    pub nb_not_acquired: i64,
    pub nb_not_acquired_percent: f64,
}

impl ComponentSummary {
    pub fn all(db: &PgConnection, cycle: &Cycle) -> Result<Vec<ComponentSummary>> {
        let components = Component::all(db)?;

        let mut components_summary = vec![];

        #[derive(Queryable, QueryableByName, Serialize)]
        struct Stats {
            #[sql_type = "BigInt"]
            nb_competencies: i64,
            #[sql_type = "BigInt"]
            nb_evaluations: i64,
            #[sql_type = "BigInt"]
            nb_empty: i64,
            #[sql_type = "BigInt"]
            nb_in_progress: i64,
            #[sql_type = "BigInt"]
            nb_acquired: i64,
            #[sql_type = "BigInt"]
            nb_not_acquired: i64,
        }

        for component in components.iter() {
            let stats: Stats = sql_query(
                "
SELECT

    (SELECT COUNT(id) FROM competencies WHERE component_id = $1)
    nb_competencies,

    (
        (SELECT COUNT(id) FROM competencies WHERE component_id = $1)
        *
        (SELECT COUNT(id) FROM eleves WHERE cycle = $2)
    )
    nb_evaluations,

    (SELECT COUNT(evaluations.id)
        FROM evaluations
        WHERE
            evaluations.competency_id IN 
                (SELECT competencies.id
                    FROM components
                        LEFT JOIN competencies
                            ON components.id = competencies.component_id
                        WHERE components.id = $1)
            AND evaluations.eleve_id IN
                (SELECT eleves.id
                    FROM eleves
                        WHERE eleves.cycle = $2)
            AND status = 'Empty')
    nb_empty,

    (SELECT COUNT(evaluations.id)
        FROM evaluations
        WHERE
            evaluations.competency_id IN 
                (SELECT competencies.id
                    FROM components
                        LEFT JOIN competencies
                            ON components.id = competencies.component_id
                        WHERE components.id = $1)
            AND evaluations.eleve_id IN
                (SELECT eleves.id
                    FROM eleves
                        WHERE eleves.cycle = $2)
            AND status = 'InProgress')
    nb_in_progress,

    (SELECT COUNT(evaluations.id)
        FROM evaluations
        WHERE
            evaluations.competency_id IN 
                (SELECT competencies.id
                    FROM components
                        LEFT JOIN competencies
                            ON components.id = competencies.component_id
                        WHERE components.id = $1)
            AND evaluations.eleve_id IN
                (SELECT eleves.id
                    FROM eleves
                        WHERE eleves.cycle = $2)
            AND status = 'Acquired')
    nb_acquired,

    (SELECT COUNT(evaluations.id)
        FROM evaluations
        WHERE
            evaluations.competency_id IN 
                (SELECT competencies.id
                    FROM components
                        LEFT JOIN competencies
                            ON components.id = competencies.component_id
                        WHERE components.id = $1)
            AND evaluations.eleve_id IN
                (SELECT eleves.id
                    FROM eleves
                        WHERE eleves.cycle = $2)
            AND status = 'NotAcquired')
    nb_not_acquired
           ",
            )
            .bind::<Integer, _>(component.id)
            .bind::<Text, _>(cycle.to_str())
            .get_result(db)?;

            components_summary.push(ComponentSummary {
                id: component.id,
                domain_id: component.domain_id,
                rank: component.rank,
                title: component.title.clone(),
                nb_competencies: stats.nb_competencies,
                nb_evaluations: stats.nb_evaluations,
                nb_empty: stats.nb_empty,
                nb_empty_percent: stats.nb_empty as f64 * 100.0 / stats.nb_evaluations as f64,
                nb_in_progress: stats.nb_in_progress,
                nb_in_progress_percent: stats.nb_in_progress as f64 * 100.0
                    / stats.nb_evaluations as f64,
                nb_acquired: stats.nb_acquired,
                nb_acquired_percent: stats.nb_acquired as f64 * 100.0 / stats.nb_evaluations as f64,
                nb_not_acquired: stats.nb_not_acquired,
                nb_not_acquired_percent: stats.nb_not_acquired as f64 * 100.0
                    / stats.nb_evaluations as f64,
            });
        }
        Ok(components_summary)
    }
}
