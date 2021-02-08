use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::cycles::Cycle;
use crate::db::evaluations::{Detail, Evaluation, Statistics, Toc};
use crate::db_log::Logger;
use crate::{DbConn, Result};

#[get("/by_id/<id>")]
pub fn by_id(db: DbConn, id: i32) -> Result<JsonValue> {
    Ok(json!({"evaluation": Evaluation::by_id(&db, id)?}))
}

#[get("/by_eleve/<eleve_id>")]
pub fn by_eleve(db: DbConn, eleve_id: i32) -> Result<JsonValue> {
    Ok(json!({"evaluations": Evaluation::by_eleve(&db, eleve_id)?}))
}

#[get("/toc/<cycle>")]
pub fn toc(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"toc": Toc::toc(&db, &Cycle::from_text(&cycle))?}))
}

#[get("/detail/<cycle>/<component_id>")]
pub fn detail(db: DbConn, cycle: String, component_id: i32) -> Result<JsonValue> {
    Ok(json!({"detail": Detail::detail(&db, &Cycle::from_text(&cycle), component_id)?}))
}

#[get("/stats")]
pub fn stats(db: DbConn) -> Result<JsonValue> {
    Ok(json!({
        "c1": Statistics::by_cycle(&db, Cycle::C1)?,
        "c2": Statistics::by_cycle(&db, Cycle::C2)?,
        "c3": Statistics::by_cycle(&db, Cycle::C3)?,
        "c4": Statistics::by_cycle(&db, Cycle::C4)?}))
}

#[get("/cycle/<cycle>/competency/<competency_id>")]
pub fn by_cycle_and_competency(db: DbConn, cycle: String, competency_id: i32) -> Result<JsonValue> {
    Ok(
        json!({"evaluations": Evaluation::by_cycle_and_competency(&db, Cycle::from_text(&cycle), competency_id)?}),
    )
}

#[derive(Deserialize, Debug)]
pub struct UpdateStatus {
    status: String,
}

#[put("/set_status/<id>", format = "json", data = "<data>")]
pub fn set_status(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateStatus>,
) -> Result<JsonValue> {
    match data.status.as_str() {
        "Empty" => Evaluation::set_empty_status(&db, &logger, id),
        "InProgress" => Evaluation::set_in_progress_status(&db, &logger, id),
        "Acquired" => Evaluation::set_acquired_status(&db, &logger, id),
        "NotAcquired" => Evaluation::set_not_acquired_status(&db, &logger, id),
        _ => panic!(format!("Invalid status: {}", &data.status)),
    }?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateComment {
    comment: String,
}

#[put("/set_comment/<id>", format = "json", data = "<data>")]
pub fn set_comment(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateComment>,
) -> Result<JsonValue> {
    Evaluation::set_comment(&db, &logger, id, data.comment.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateSingleStatus {
    id: i32,
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateStatuses {
    statuses: Vec<UpdateSingleStatus>,
}

#[put("/set_statuses", format = "json", data = "<data>")]
pub fn set_statuses(
    db: DbConn,
    logger: State<Logger>,
    data: Json<UpdateStatuses>,
) -> Result<JsonValue> {
    for status in data.statuses.iter() {
        match status.status.as_str() {
            "Empty" => Evaluation::set_empty_status(&db, &logger, status.id),
            "InProgress" => Evaluation::set_in_progress_status(&db, &logger, status.id),
            "Acquired" => Evaluation::set_acquired_status(&db, &logger, status.id),
            "NotAcquired" => Evaluation::set_not_acquired_status(&db, &logger, status.id),
            _ => panic!(format!("Invalid status: {}", &status.status)),
        }?;
    }
    Ok(json!({"done": true}))
}
