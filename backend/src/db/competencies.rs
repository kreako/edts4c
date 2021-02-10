use crate::db::cycles::Cycle;
use crate::db_log::Logger;
use crate::Result;
use diesel;
use diesel::dsl::max;
use diesel::PgConnection;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Competency {
    pub id: i32,
    pub component_id: i32,
    pub rank: i32,
    pub title: String,
    pub c1: Option<String>,
    pub c2: Option<String>,
    pub c3: Option<String>,
    pub c4: Option<String>,
}

impl Competency {
    pub fn detail(&self, cycle: &Cycle) -> Option<String> {
        match cycle {
            Cycle::C1 => self.c1.clone(),
            Cycle::C2 => self.c2.clone(),
            Cycle::C3 => self.c3.clone(),
            Cycle::C4 => self.c4.clone(),
        }
    }

    pub fn by_id(db: &PgConnection, id: i32) -> Result<Competency> {
        use crate::schema::competencies::dsl;
        let competency = dsl::competencies.find(id).get_result::<Competency>(db)?;
        Ok(competency)
    }

    pub fn by_component_id(db: &PgConnection, component_id: i32) -> Result<Vec<Competency>> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        let competencies = dsl::competencies
            .order(competencies::rank)
            .filter(competencies::component_id.eq(component_id))
            .load::<Competency>(db);
        Ok(competencies?)
    }

    pub fn all(db: &PgConnection) -> Result<Vec<Competency>> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        Ok(dsl::competencies
            .order(competencies::rank)
            .load::<Competency>(db)?)
    }

    pub fn next_competency_id(
        db: &PgConnection,
        component_id: i32,
        rank: i32,
    ) -> Result<Option<i32>> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        let id = dsl::competencies
            .select(competencies::id)
            .order(competencies::rank)
            .filter(competencies::rank.gt(rank))
            .filter(competencies::component_id.eq(component_id))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    pub fn previous_competency_id(
        db: &PgConnection,
        component_id: i32,
        rank: i32,
    ) -> Result<Option<i32>> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        let id = dsl::competencies
            .select(competencies::id)
            .order(competencies::rank.desc())
            .filter(competencies::rank.lt(rank))
            .filter(competencies::component_id.eq(component_id))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    /// Return the list of all competency id
    pub fn all_id(db: &PgConnection) -> Result<Vec<i32>> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        Ok(dsl::competencies.select(competencies::id).load::<i32>(db)?)
    }

    pub fn update_cycle(
        db: &PgConnection,
        log: &Logger,
        competency_id: i32,
        cycle: Cycle,
        text: String,
    ) -> Result<()> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        match cycle {
            Cycle::C1 => {
                // TODO check with get_result if this is succesful ?
                diesel::update(dsl::competencies.find(competency_id))
                    .set(competencies::c1.eq(Some(text.clone())))
                    .execute(db)?;
                log.update1(
                    "competencies".to_string(),
                    competency_id,
                    "c1".to_string(),
                    text,
                )?;
            }
            Cycle::C2 => {
                diesel::update(dsl::competencies.find(competency_id))
                    .set(competencies::c2.eq(Some(text.clone())))
                    .execute(db)?;
                log.update1(
                    "competencies".to_string(),
                    competency_id,
                    "c2".to_string(),
                    text,
                )?;
            }
            Cycle::C3 => {
                diesel::update(dsl::competencies.find(competency_id))
                    .set(competencies::c3.eq(Some(text.clone())))
                    .execute(db)?;
                log.update1(
                    "competencies".to_string(),
                    competency_id,
                    "c3".to_string(),
                    text,
                )?;
            }
            Cycle::C4 => {
                diesel::update(dsl::competencies.find(competency_id))
                    .set(competencies::c4.eq(Some(text.clone())))
                    .execute(db)?;
                log.update1(
                    "competencies".to_string(),
                    competency_id,
                    "c4".to_string(),
                    text,
                )?;
            }
        };
        Ok(())
    }

    pub fn set_title(db: &PgConnection, log: &Logger, id: i32, title: String) -> Result<()> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        diesel::update(dsl::competencies.find(id))
            .set(competencies::title.eq(title.clone()))
            .execute(db)?;
        log.update1("competencies".to_string(), id, "title".to_string(), title)?;
        Ok(())
    }

    pub fn set_rank(db: &PgConnection, id: i32, rank: i32) -> Result<()> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        diesel::update(dsl::competencies.find(id))
            .set(competencies::rank.eq(rank))
            .execute(db)?;
        Ok(())
    }

    pub fn set_component_id(db: &PgConnection, id: i32, component_id: i32) -> Result<()> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        diesel::update(dsl::competencies.find(id))
            .set(competencies::component_id.eq(component_id))
            .execute(db)?;
        Ok(())
    }

    pub fn set_competencies_rank(db: &PgConnection, competencies: Vec<Competency>) -> Result<()> {
        // competencies array arrives sorted as the user wants it
        // with invalid rank inside that I need to set
        let mut rank = 1;
        for competency in competencies {
            Competency::set_rank(db, competency.id, rank)?;
            rank += 1;
        }
        Ok(())
    }

    pub fn max_rank(db: &PgConnection, component_id: i32) -> Result<i32> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        // Select the current max rank
        let max_rank: Option<i32> = dsl::competencies
            .select(max(competencies::rank))
            .filter(competencies::component_id.eq(component_id))
            .first(db)?;
        Ok(max_rank.unwrap_or(0))
    }

    /// Return the new Competency
    pub fn append(db: &PgConnection, component_id: i32, title: String) -> Result<Competency> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        // Compute the needed rank
        let rank: i32 = Competency::max_rank(db, component_id)? + 1;
        // insert
        let competency_id: i32 = diesel::insert_into(dsl::competencies)
            .values((
                competencies::component_id.eq(component_id),
                competencies::rank.eq(rank),
                competencies::title.eq(title),
                competencies::c1.eq::<Option<String>>(None),
                competencies::c2.eq::<Option<String>>(None),
                competencies::c3.eq::<Option<String>>(None),
                competencies::c4.eq::<Option<String>>(None),
            ))
            .returning(competencies::id)
            .get_result(db)?;
        // return the new one
        Competency::by_id(db, competency_id)
    }

    pub fn delete(db: &PgConnection, id: i32) -> Result<()> {
        use crate::schema::competencies;
        use crate::schema::competencies::dsl;
        let competency = Competency::by_id(db, id)?;
        // Delete
        diesel::delete(dsl::competencies.filter(competencies::id.eq(id))).execute(db)?;
        // Fix rank of the component_id
        Competency::fix_rank(db, competency.component_id)?;
        Ok(())
    }

    pub fn fix_rank(db: &PgConnection, component_id: i32) -> Result<()> {
        let competencies = Competency::by_component_id(db, component_id)?;
        Competency::set_competencies_rank(db, competencies)?;
        Ok(())
    }

    pub fn move_to_component(
        db: &PgConnection,
        competency_id: i32,
        component_id: i32,
    ) -> Result<()> {
        let competency = Competency::by_id(db, competency_id)?;
        // Compute the needed rank in the next component
        let rank: i32 = Competency::max_rank(db, component_id)? + 1;
        // Update
        Competency::set_rank(db, competency_id, rank)?;
        Competency::set_component_id(db, competency_id, component_id)?;
        // Fix rank of the previous component_id
        Competency::fix_rank(db, competency.component_id)?;
        Ok(())
    }
}
