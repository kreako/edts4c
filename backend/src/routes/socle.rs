use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use crate::db::socle::Socle;
use crate::{DbConn, Result};

/// Get socle
#[get("/toc")]
pub fn toc(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"toc": Socle::toc(&db)?}))
}

/// Get component
#[get("/component/<id>")]
pub fn component(db: DbConn, id: i32) -> Result<JsonValue> {
    Ok(json!({"component": Socle::component(&db, id)?}))
}
