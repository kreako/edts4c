use crate::db::competencies::Competency;
use crate::db::components::{Component, ComponentSummary};
use crate::db::cycles::Cycle;
use crate::db::domains::Domain;
use crate::db::eleves::Eleve;
use crate::db_log::Logger;
use crate::Result;
use diesel::sql_types::{BigInt, Integer, Nullable, Text};
use diesel::{sql_query, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, QueryableByName, Serialize)]
pub struct Evaluation {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub eleve_id: i32,
    #[sql_type = "Integer"]
    pub competency_id: i32,
    #[sql_type = "Text"]
    pub status: String,
    #[sql_type = "Nullable<Text>"]
    pub comment: Option<String>,
}

impl Evaluation {
    pub fn by_id(db: &PgConnection, id: i32) -> Result<Evaluation> {
        use crate::schema::evaluations::dsl;
        let evaluation = dsl::evaluations.find(id).first::<Evaluation>(db)?;
        Ok(evaluation)
    }

    pub fn by_eleve(db: &PgConnection, eleve_id: i32) -> Result<Vec<Evaluation>> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        let evaluations = dsl::evaluations
            .filter(evaluations::eleve_id.eq(eleve_id))
            .load::<Evaluation>(db)?;
        Ok(evaluations)
    }

    pub fn by_cycle_and_competency(
        db: &PgConnection,
        cycle: Cycle,
        competency_id: i32,
    ) -> Result<Vec<Evaluation>> {
        let evaluations = sql_query(
            "
SELECT id, eleve_id, competency_id, status, comment
    FROM evaluations
    WHERE
        eleve_id in (SELECT id FROM eleves WHERE cycle = $1)
        AND
        competency_id = $2",
        )
        .bind::<Text, _>(cycle.to_str())
        .bind::<Integer, _>(competency_id)
        .load::<Evaluation>(db)?;
        Ok(evaluations)
    }

    pub fn by_eleve_and_competency(
        db: &PgConnection,
        eleve_id: i32,
        competency_id: i32,
    ) -> Result<Evaluation> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        let evaluation = dsl::evaluations
            .filter(evaluations::eleve_id.eq(eleve_id))
            .filter(evaluations::competency_id.eq(competency_id))
            .first(db)?;
        Ok(evaluation)
    }

    /// Create empty evaluation for an eleve
    pub fn create_empty_for_eleve(db: &PgConnection, eleve_id: i32) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        let mut data = vec![];
        let competencies = Competency::all_id(db)?;
        for competency_id in competencies.iter() {
            data.push((
                evaluations::eleve_id.eq(eleve_id),
                evaluations::competency_id.eq(competency_id),
                evaluations::status.eq("Empty".to_string()),
            ));
        }
        diesel::insert_into(dsl::evaluations)
            .values(&data)
            .execute(db)?;
        Ok(())
    }

    /// Set empty status of an evaluation
    pub fn set_empty_status(db: &PgConnection, log: &Logger, id: i32) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        diesel::update(dsl::evaluations.find(id))
            .set(evaluations::status.eq("Empty".to_string()))
            .execute(db)?;
        log.update1(
            "evaluations".to_string(),
            id,
            "status".to_string(),
            "Empty".to_string(),
        )?;
        Ok(())
    }

    /// Set in progress status of an evaluation
    pub fn set_in_progress_status(db: &PgConnection, log: &Logger, id: i32) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        diesel::update(dsl::evaluations.find(id))
            .set(evaluations::status.eq("InProgress".to_string()))
            .execute(db)?;
        log.update1(
            "evaluations".to_string(),
            id,
            "status".to_string(),
            "InProgress".to_string(),
        )?;
        Ok(())
    }

    /// Set acquired status of an evaluation
    pub fn set_acquired_status(db: &PgConnection, log: &Logger, id: i32) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        diesel::update(dsl::evaluations.find(id))
            .set(evaluations::status.eq("Acquired".to_string()))
            .execute(db)?;
        log.update1(
            "evaluations".to_string(),
            id,
            "status".to_string(),
            "Acquired".to_string(),
        )?;
        Ok(())
    }

    /// Set empty status of an evaluation
    pub fn set_not_acquired_status(db: &PgConnection, log: &Logger, id: i32) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        diesel::update(dsl::evaluations.find(id))
            .set(evaluations::status.eq("NotAcquired".to_string()))
            .execute(db)?;
        log.update1(
            "evaluations".to_string(),
            id,
            "status".to_string(),
            "NotAcquired".to_string(),
        )?;
        Ok(())
    }

    /// Set comment of an evaluation
    pub fn set_comment(db: &PgConnection, log: &Logger, id: i32, comment: String) -> Result<()> {
        use crate::schema::evaluations;
        use crate::schema::evaluations::dsl;
        diesel::update(dsl::evaluations.find(id))
            .set(evaluations::comment.eq(comment.clone()))
            .execute(db)?;
        log.update1(
            "evaluations".to_string(),
            id,
            "comment".to_string(),
            comment,
        )?;
        Ok(())
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct Statistics {
    #[sql_type = "BigInt"]
    pub nb_eleves: i64,
    #[sql_type = "BigInt"]
    pub count: i64,
    #[sql_type = "BigInt"]
    pub empty: i64,
    #[sql_type = "BigInt"]
    pub in_progress: i64,
    #[sql_type = "BigInt"]
    pub acquired: i64,
    #[sql_type = "BigInt"]
    pub not_acquired: i64,
}

impl Statistics {
    pub fn by_cycle(db: &PgConnection, cycle: Cycle) -> Result<Statistics> {
        let stats = sql_query(
            "
SELECT 

    (SELECT COUNT(id) FROM eleves WHERE cycle = $1)
    nb_eleves,

    (SELECT COUNT(id) FROM evaluations
        WHERE eleve_id in (SELECT id FROM eleves WHERE cycle = $1))
    count,

    (SELECT COUNT(id) FROM evaluations
        WHERE eleve_id in (SELECT id FROM eleves WHERE cycle = $1)
        AND status = 'Empty')
    empty,

    (SELECT COUNT(id) FROM evaluations
        WHERE eleve_id in (SELECT id FROM eleves WHERE cycle = $1)
        AND status = 'InProgress')
    in_progress,

    (SELECT COUNT(id) FROM evaluations
        WHERE eleve_id in (SELECT id FROM eleves WHERE cycle = $1)
        AND status = 'Acquired')
    acquired,

    (SELECT COUNT(id) FROM evaluations
        WHERE eleve_id in (SELECT id FROM eleves WHERE cycle = $1)
        AND status = 'NotAcquired')
    not_acquired",
        )
        .bind::<Text, _>(cycle.to_str())
        .get_result(db)?;
        Ok(stats)
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct ByComponentStatistics {
    #[sql_type = "BigInt"]
    pub nb_in_progress: i64,
    #[sql_type = "BigInt"]
    pub nb_acquired: i64,
    #[sql_type = "BigInt"]
    pub nb_not_acquired: i64,
}

impl ByComponentStatistics {
    pub fn stats(
        db: &PgConnection,
        component_id: i32,
        eleve_id: i32,
    ) -> Result<ByComponentStatistics> {
        let stats = sql_query(
            "
SELECT

    (SELECT COUNT(id)
        FROM evaluations
        WHERE
            competency_id in (SELECT id FROM competencies WHERE component_id = $1)
            AND eleve_id = $2
            AND  status = 'InProgress')
    nb_in_progress,

    (SELECT COUNT(id)
        FROM evaluations
        WHERE
            competency_id in (SELECT id FROM competencies WHERE component_id = $1)
            AND eleve_id = $2
            AND  status = 'Acquired')
    nb_acquired,

    (SELECT COUNT(id)
        FROM evaluations
        WHERE
            competency_id in (SELECT id FROM competencies WHERE component_id = $1)
            AND eleve_id = $2
            AND  status = 'NotAcquired')
    nb_not_acquired
        ",
        )
        .bind::<Integer, _>(component_id)
        .bind::<Integer, _>(eleve_id)
        .get_result(db)?;
        Ok(stats)
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct ByComponentComment {
    #[sql_type = "Text"]
    pub comment: String,
}

impl ByComponentComment {
    pub fn comments(
        db: &PgConnection,
        component_id: i32,
        eleve_id: i32,
    ) -> Result<Vec<ByComponentComment>> {
        let comments = sql_query(
            "
SELECT comment
    FROM evaluations
    WHERE
        competency_id in (SELECT id FROM competencies WHERE component_id = $1)
        AND eleve_id = $2
        AND comment is not null
        ",
        )
        .bind::<Integer, _>(component_id)
        .bind::<Integer, _>(eleve_id)
        .get_results(db)?;
        Ok(comments)
    }
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub enum Toc {
    Domain { data: Domain },
    Component { data: ComponentSummary },
}

impl Toc {
    pub fn toc(db: &PgConnection, cycle: &Cycle) -> Result<Vec<Toc>> {
        let mut toc = vec![];
        let domains = Domain::all(db)?;
        let components = ComponentSummary::all(db, cycle)?;
        for domain in domains.iter() {
            toc.push(Toc::Domain {
                data: (*domain).clone(),
            });
            for component in components.iter() {
                if component.domain_id == domain.id {
                    toc.push(Toc::Component {
                        data: (*component).clone(),
                    });
                }
            }
        }
        Ok(toc)
    }
}

#[derive(Serialize)]
pub struct Detail {
    pub domain: Domain,
    pub component: Component,
    pub next_component_id: Option<i32>,
    pub competencies: Vec<DetailCompetency>,
}

#[derive(Serialize)]
pub struct DetailCompetency {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub detail: Option<String>,
    pub evaluations: Vec<DetailEvaluation>,
}

#[derive(Serialize)]
pub struct DetailEvaluation {
    pub id: i32,
    pub eleve_firstname: String,
    pub eleve_lastname: String,
    pub status: String,
    pub comment: Option<String>,
}

impl Detail {
    pub fn detail(db: &PgConnection, cycle: &Cycle, component_id: i32) -> Result<Detail> {
        let component = Component::by_id(db, component_id)?;
        let ordered_components = Component::all_ordered(db)?;
        let domain = Domain::by_id(db, component.domain_id)?;
        let competencies = Competency::by_component_id(db, component_id)?;
        let eleves = Eleve::by_cycle(db, cycle)?;
        let mut next_component_id = None;
        let mut next = false;
        for c in ordered_components.iter() {
            if next {
                next_component_id = Some(c.id);
                break;
            }
            if c.id == component.id {
                next = true;
            }
        }
        let mut detail = Detail {
            domain: domain.clone(),
            component: component.clone(),
            next_component_id: next_component_id,
            competencies: vec![],
        };
        for competency in competencies.iter() {
            let mut detail_competency = DetailCompetency {
                id: competency.id,
                rank: competency.id,
                title: competency.title.clone(),
                detail: competency.detail(cycle).clone(),
                evaluations: vec![],
            };
            for eleve in eleves.iter() {
                let evaluation = Evaluation::by_eleve_and_competency(db, eleve.id, competency.id)?;
                let detail_evaluation = DetailEvaluation {
                    id: evaluation.id,
                    eleve_firstname: eleve.firstname.clone(),
                    eleve_lastname: eleve.lastname.clone(),
                    status: evaluation.status.clone(),
                    comment: evaluation.comment.clone(),
                };
                detail_competency.evaluations.push(detail_evaluation);
            }
            detail.competencies.push(detail_competency);
        }
        Ok(detail)
    }
}
