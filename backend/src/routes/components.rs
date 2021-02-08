use crate::db_log::Logger;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::components::{Component, ComponentSummary};
use crate::db::cycles::Cycle;
use crate::db::domains::Domain;
use crate::{DbConn, Result};

/// Get components by domain id
#[get("/by_domain_id/<domain_id>")]
pub fn by_domain_id(db: DbConn, domain_id: i32) -> Result<JsonValue> {
    let domain = Domain::by_id(&db, domain_id)?;
    let next_domain_id = Domain::next_domain_id(&db, domain.rank)?;
    let previous_domain_id = Domain::previous_domain_id(&db, domain.rank)?;
    Ok(
        json!({"components": Component::by_domain_id(&db, domain_id)?, 
            "domain": domain,
            "next_domain_id": next_domain_id,
            "previous_domain_id": previous_domain_id, }),
    )
}

/// Get all components
#[get("/all")]
pub fn all(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"components": Component::all(&db)?}))
}

/// Get all components ordered
#[get("/all_ordered")]
pub fn all_ordered(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"components": Component::all_ordered(&db)?}))
}

/// Get all component summary
#[get("/summary/<cycle>")]
pub fn summary(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"components": ComponentSummary::all(&db, &Cycle::from_text(&cycle))?}))
}

#[derive(Deserialize, Debug)]
pub struct UpdateTitle {
    title: String,
}

/// Set components title
#[put("/set_title/<id>", format = "json", data = "<data>")]
pub fn set_title(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateTitle>,
) -> Result<JsonValue> {
    Component::set_title(&db, &logger, id, data.title.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManyComponents {
    pub components: Vec<Component>,
}

#[put("/set_components_rank", format = "json", data = "<data>")]
pub fn set_components_rank(db: DbConn, data: Json<ManyComponents>) -> Result<JsonValue> {
    Component::set_components_rank(&db, data.components.clone())?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppendComponent {
    pub title: String,
    pub domain_id: i32,
}

#[post("/append", format = "json", data = "<data>")]
pub fn append(db: DbConn, data: Json<AppendComponent>) -> Result<JsonValue> {
    Ok(json!({"component": Component::append(&db, data.domain_id, data.title.clone())?}))
}

#[delete("/delete/<id>")]
pub fn delete(db: DbConn, id: i32) -> Result<JsonValue> {
    Component::delete(&db, id)?;
    Ok(json!({"done": true}))
}

#[derive(Deserialize, Debug, Clone)]
pub struct MoveComponent {
    pub component_id: i32,
    pub domain_id: i32,
}

#[put("/move_to_domain", format = "json", data = "<data>")]
pub fn move_to_domain(db: DbConn, data: Json<MoveComponent>) -> Result<JsonValue> {
    Component::move_to_domain(&db, data.component_id, data.domain_id)?;
    Ok(json!({"done": true}))
}
