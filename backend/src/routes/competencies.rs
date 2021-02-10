use crate::db_log::Logger;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::competencies::Competency;
use crate::db::components::Component;
use crate::db::cycles::Cycle;
use crate::db::domains::Domain;
use crate::{DbConn, Result};

/// Get competencies by component id
#[get("/by_component_id/<component_id>")]
pub fn by_component_id(db: DbConn, component_id: i32) -> Result<JsonValue> {
    let component = Component::by_id(&db, component_id)?;
    let next_component_id = Component::next_component_id(&db, component.domain_id, component.rank)?;
    let previous_component_id =
        Component::previous_component_id(&db, component.domain_id, component.rank)?;
    let domain = Domain::by_id(&db, component.domain_id)?;
    Ok(json!({
        "competencies": Competency::by_component_id(&db, component_id)?,
        "component": component,
        "next_component_id": next_component_id,
        "previous_component_id": previous_component_id,
        "domain": domain,
    }))
}

#[get("/by_id/<competency_id>")]
pub fn by_id(db: DbConn, competency_id: i32) -> Result<JsonValue> {
    let competency = Competency::by_id(&db, competency_id)?;
    let next_competency_id =
        Competency::next_competency_id(&db, competency.component_id, competency.rank)?;
    let previous_competency_id =
        Competency::previous_competency_id(&db, competency.component_id, competency.rank)?;
    let component = Component::by_id(&db, competency.component_id)?;
    let domain = Domain::by_id(&db, component.domain_id)?;
    Ok(json!({
        "competency": competency,
        "next_competency_id": next_competency_id,
        "previous_competency_id": previous_competency_id,
        "component": component,
        "domain": domain,
    }))
}

/// All competencies
#[get("/all")]
pub fn all(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"competencies": Competency::all(&db)?}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateCycle {
    text: String,
}

/// Put competency cycle detail text for C1
#[put("/update_cycle/c1/<competency_id>", format = "json", data = "<data>")]
pub fn update_cycle_c1(
    db: DbConn,
    logger: State<Logger>,
    competency_id: i32,
    data: Json<UpdateCycle>,
) -> Result<JsonValue> {
    Competency::update_cycle(&db, &logger, competency_id, Cycle::C1, data.text.clone())?;
    Ok(json!({"done": true}))
}

/// Put competency cycle detail text for C2
#[put("/update_cycle/c2/<competency_id>", format = "json", data = "<data>")]
pub fn update_cycle_c2(
    db: DbConn,
    logger: State<Logger>,
    competency_id: i32,
    data: Json<UpdateCycle>,
) -> Result<JsonValue> {
    Competency::update_cycle(&db, &logger, competency_id, Cycle::C2, data.text.clone())?;
    Ok(json!({"done": true}))
}

/// Put competency cycle detail text for C3
#[put("/update_cycle/c3/<competency_id>", format = "json", data = "<data>")]
pub fn update_cycle_c3(
    db: DbConn,
    logger: State<Logger>,
    competency_id: i32,
    data: Json<UpdateCycle>,
) -> Result<JsonValue> {
    Competency::update_cycle(&db, &logger, competency_id, Cycle::C3, data.text.clone())?;
    Ok(json!({"done": true}))
}

/// Put competency cycle detail text for C4
#[put("/update_cycle/c4/<competency_id>", format = "json", data = "<data>")]
pub fn update_cycle_c4(
    db: DbConn,
    logger: State<Logger>,
    competency_id: i32,
    data: Json<UpdateCycle>,
) -> Result<JsonValue> {
    Competency::update_cycle(&db, &logger, competency_id, Cycle::C4, data.text.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateTitle {
    title: String,
}

/// Set competency title
#[put("/set_title/<id>", format = "json", data = "<data>")]
pub fn set_title(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateTitle>,
) -> Result<JsonValue> {
    Competency::set_title(&db, &logger, id, data.title.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManyCompetencies {
    pub competencies: Vec<Competency>,
}

#[put("/set_competencies_rank", format = "json", data = "<data>")]
pub fn set_competencies_rank(db: DbConn, data: Json<ManyCompetencies>) -> Result<JsonValue> {
    Competency::set_competencies_rank(&db, data.competencies.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppendCompetency {
    pub title: String,
    pub component_id: i32,
}

#[post("/append", format = "json", data = "<data>")]
pub fn append(db: DbConn, data: Json<AppendCompetency>) -> Result<JsonValue> {
    Ok(json!({"competency": Competency::append(&db, data.component_id, data.title.clone())?}))
}

#[delete("/delete/<id>")]
pub fn delete(db: DbConn, id: i32) -> Result<JsonValue> {
    Competency::delete(&db, id)?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct MoveCompetency {
    pub competency_id: i32,
    pub component_id: i32,
}

#[put("/move_to_component", format = "json", data = "<data>")]
pub fn move_to_component(db: DbConn, data: Json<MoveCompetency>) -> Result<JsonValue> {
    Competency::move_to_component(&db, data.competency_id, data.component_id)?;
    Ok(json!({"done": true}))
}
