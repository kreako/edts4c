use crate::db_log::Logger;
use chrono::NaiveDate;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::cycles::Cycle;
use crate::db::eleves::{CycleEleve, Eleve};
use crate::{DbConn, Result};

/// Get all eleves
#[get("/all")]
pub fn all(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"eleves": CycleEleve::all(&db)?}))
}

/// Get all eleves by a cycle
#[get("/by_cycle/<cycle>")]
pub fn by_cycle(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"eleves": Eleve::by_cycle(&db, &Cycle::from_text(&cycle))?}))
}

/// Get one eleve by id
#[get("/by_id/<id>")]
pub fn by_id(db: DbConn, id: i32) -> Result<JsonValue> {
    Ok(json!({"eleve": CycleEleve::by_id(&db, id)?}))
}

#[derive(Deserialize, Debug)]
pub struct NewEleve {
    firstname: String,
    lastname: String,
    birthdate: NaiveDate,
    school_entry: NaiveDate,
    cycle: String,
}

/// New eleve
#[post("/new", format = "json", data = "<data>")]
pub fn new(db: DbConn, logger: State<Logger>, data: Json<NewEleve>) -> Result<JsonValue> {
    Eleve::new(
        &db,
        &logger,
        data.firstname.clone(),
        data.lastname.clone(),
        data.birthdate.clone(),
        data.school_entry.clone(),
        Cycle::from_text(&data.cycle),
    )?;
    Ok(json!({"done": true}))
}

/// update eleve
#[put("/update/<id>", format = "json", data = "<data>")]
pub fn update(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<NewEleve>,
) -> Result<JsonValue> {
    Eleve::update(
        &db,
        &logger,
        id,
        data.firstname.clone(),
        data.lastname.clone(),
        data.birthdate.clone(),
        data.school_entry.clone(),
        Cycle::from_text(&data.cycle),
    )?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateActive {
    active: bool,
}

/// Set eleve active
#[put("/set_active/<id>", format = "json", data = "<data>")]
pub fn set_active(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateActive>,
) -> Result<JsonValue> {
    Eleve::set_active(&db, &logger, id, data.active)?;
    Ok(json!({"done": true}))
}
