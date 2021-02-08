use crate::db::competencies::Competency;
use crate::db::components;
use crate::db::domains;
use crate::Result;
use diesel::PgConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct Domain {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub components: Vec<components::Component>,
}

#[derive(Serialize)]
pub struct Component {
    pub id: i32,
    pub rank: i32,
    pub title: String,
    pub previous_component_id: Option<i32>,
    pub next_component_id: Option<i32>,
    pub competencies: Vec<Competency>,
}

pub struct Socle;

impl Socle {
    pub fn toc(db: &PgConnection) -> Result<Vec<Domain>> {
        let mut socle = vec![];
        let domains = domains::Domain::all(db)?;
        let components = components::Component::all(db)?;
        for d in domains.iter() {
            let mut domain = Domain {
                id: d.id,
                rank: d.rank,
                title: d.title.clone(),
                components: vec![],
            };
            for c in components.iter() {
                if c.domain_id == domain.id {
                    domain.components.push(c.clone());
                }
            }
            socle.push(domain);
        }
        Ok(socle)
    }

    pub fn component(db: &PgConnection, component_id: i32) -> Result<Component> {
        let component = components::Component::by_id(db, component_id)?;
        let competencies = Competency::by_component_id(db, component_id)?;
        let components = components::Component::all_ordered(db)?;
        let mut next = false;
        let mut previous = None;
        let mut previous_component_id = None;
        let mut next_component_id = None;
        for c in components.iter() {
            if next {
                next_component_id = Some(c.id);
                break;
            }
            if c.id == component_id {
                previous_component_id = previous;
                next = true;
            }
            previous = Some(c.id);
        }
        Ok(Component {
            id: component.id,
            rank: component.rank,
            title: component.title.clone(),
            previous_component_id: previous_component_id,
            next_component_id: next_component_id,
            competencies: competencies,
        })
    }
}
