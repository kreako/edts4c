use crate::db_log::Logger;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::domains::Domain;
use crate::{DbConn, Result};

/// Get all domains
#[get("/all")]
pub fn all(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"domains": Domain::all(&db)?}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateTitle {
    title: String,
}

/// Set domains title
#[put("/set_title/<id>", format = "json", data = "<data>")]
pub fn set_title(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateTitle>,
) -> Result<JsonValue> {
    Domain::set_title(&db, &logger, id, data.title.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct AllDomains {
    pub domains: Vec<Domain>,
}

#[put("/set_domains_rank", format = "json", data = "<data>")]
pub fn set_domains_rank(db: DbConn, data: Json<AllDomains>) -> Result<JsonValue> {
    Domain::set_domains_rank(&db, data.domains.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppendDomain {
    pub title: String,
}

#[post("/append", format = "json", data = "<data>")]
pub fn append(db: DbConn, data: Json<AppendDomain>) -> Result<JsonValue> {
    Ok(json!({"domain": Domain::append(&db, data.title.clone())?}))
}

#[delete("/delete/<id>")]
pub fn delete(db: DbConn, id: i32) -> Result<JsonValue> {
    Domain::delete(&db, id)?;
    Ok(json!({"done": true}))
}
