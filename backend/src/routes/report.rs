use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use crate::db::cycles::Cycle;
use crate::db::report::Report;
use crate::{DbConn, Result};

/// Get summary report
#[get("/summary/<cycle>")]
pub fn summary(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"report": Report::summary(&db, &Cycle::from_text(&cycle))?}))
}

/// Get full report
#[get("/full/<cycle>")]
pub fn full(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"report": Report::full(&db, &Cycle::from_text(&cycle))?}))
}
