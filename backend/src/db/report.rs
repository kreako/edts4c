use crate::db::competencies::Competency;
use crate::db::components::Component;
use crate::db::cycles::Cycle;
use crate::db::domains::Domain;
use crate::db::eleves::Eleve;
use crate::db::evaluations::{ByComponentComment, ByComponentStatistics, Evaluation};
use crate::db::general_comment::GeneralComment;
use crate::db::key_store::KeyStore;
use crate::Result;
use chrono::NaiveDate;
use diesel::PgConnection;
use serde::Serialize;

pub struct Report;

#[derive(Serialize)]
pub struct Summary {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthdate: NaiveDate,
    pub school_entry: NaiveDate,
    pub evaluation_date: NaiveDate,
    pub general_comment: Option<String>,
    pub domains: Vec<DomainSummary>,
}

#[derive(Serialize)]
pub struct DomainSummary {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub components: Vec<ComponentSummary>,
}

#[derive(Serialize)]
pub struct ComponentSummary {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub nb_in_progress: i64,
    pub nb_acquired: i64,
    pub nb_not_acquired: i64,
    pub comments: Vec<String>,
}

#[derive(Serialize)]
pub struct Full {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthdate: NaiveDate,
    pub school_entry: NaiveDate,
    pub evaluation_date: NaiveDate,
    pub general_comment: Option<String>,
    pub domains: Vec<DomainFull>,
}

#[derive(Serialize)]
pub struct DomainFull {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub components: Vec<ComponentFull>,
}

#[derive(Serialize)]
pub struct ComponentFull {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub competencies: Vec<CompetencyFull>,
}

#[derive(Serialize)]
pub struct CompetencyFull {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub status: String,
    pub comment: Option<String>,
}

impl Report {
    pub fn full(db: &PgConnection, cycle: &Cycle) -> Result<Vec<Full>> {
        let mut fulls = vec![];
        let domains = Domain::all(db)?;
        let components = Component::all(db)?;
        let competencies = Competency::all(db)?;
        let eleves = Eleve::by_cycle(db, cycle)?;
        for eleve in eleves.iter() {
            let general_comment = GeneralComment::by_eleve(db, eleve.id)?;
            let mut full = Full {
                id: eleve.id,
                firstname: eleve.firstname.clone(),
                lastname: eleve.lastname.clone(),
                birthdate: eleve.birthdate.clone(),
                school_entry: eleve.school_entry.clone(),
                evaluation_date: KeyStore::get_evaluation_date(db)?,
                general_comment: general_comment.comment.clone(),
                domains: vec![],
            };
            for domain in domains.iter() {
                let mut domain_full = DomainFull {
                    id: domain.id,
                    rank: domain.rank,
                    title: domain.title.clone(),
                    components: vec![],
                };
                for component in components.iter() {
                    if component.domain_id == domain.id {
                        let mut component_full = ComponentFull {
                            id: component.id,
                            rank: component.rank,
                            title: component.title.clone(),
                            competencies: vec![],
                        };
                        for competency in competencies.iter() {
                            if competency.component_id == component.id {
                                let evaluation = Evaluation::by_eleve_and_competency(
                                    db,
                                    eleve.id,
                                    competency.id,
                                )?;
                                let competency_full = CompetencyFull {
                                    id: competency.id,
                                    rank: competency.rank,
                                    title: competency.title.clone(),
                                    status: evaluation.status.clone(),
                                    comment: evaluation.comment.clone(),
                                };
                                component_full.competencies.push(competency_full);
                            }
                        }
                        domain_full.components.push(component_full);
                    }
                }
                full.domains.push(domain_full);
            }
            fulls.push(full);
        }
        Ok(fulls)
    }

    pub fn summary(db: &PgConnection, cycle: &Cycle) -> Result<Vec<Summary>> {
        let mut summaries = vec![];
        let domains = Domain::all(db)?;
        let components = Component::all(db)?;
        let eleves = Eleve::by_cycle(db, cycle)?;
        for eleve in eleves.iter() {
            let general_comment = GeneralComment::by_eleve(db, eleve.id)?;
            let mut summary = Summary {
                id: eleve.id,
                firstname: eleve.firstname.clone(),
                lastname: eleve.lastname.clone(),
                birthdate: eleve.birthdate.clone(),
                school_entry: eleve.school_entry.clone(),
                evaluation_date: KeyStore::get_evaluation_date(db)?,
                general_comment: general_comment.comment.clone(),
                domains: vec![],
            };
            for domain in domains.iter() {
                let mut domain_summary = DomainSummary {
                    id: domain.id,
                    rank: domain.rank,
                    title: domain.title.clone(),
                    components: vec![],
                };
                for component in components.iter() {
                    if component.domain_id == domain.id {
                        let stat = ByComponentStatistics::stats(db, component.id, eleve.id)?;
                        let by_component_comments =
                            ByComponentComment::comments(db, component.id, eleve.id)?;
                        let mut comments = vec![];
                        for comment in by_component_comments.iter() {
                            comments.push(comment.comment.to_string());
                        }
                        let component_summary = ComponentSummary {
                            id: component.id,
                            rank: component.rank,
                            title: component.title.clone(),
                            nb_in_progress: stat.nb_in_progress,
                            nb_acquired: stat.nb_acquired,
                            nb_not_acquired: stat.nb_not_acquired,
                            comments: comments,
                        };
                        domain_summary.components.push(component_summary);
                    }
                }
                summary.domains.push(domain_summary);
            }
            summaries.push(summary);
        }
        Ok(summaries)
    }
}
