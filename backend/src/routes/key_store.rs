use chrono::NaiveDate;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::key_store::KeyStore;
use crate::db_log::Logger;
use crate::{DbConn, Result};

#[get("/evaluation_date")]
pub fn evaluation_date(db: DbConn) -> Result<JsonValue> {
    Ok(json!({"evaluation_date": KeyStore::get_evaluation_date(&db)?}))
}

#[derive(Deserialize, Debug)]
pub struct SetEvaluationDate {
    evaluation_date: NaiveDate,
}

#[put("/evaluation_date", format = "json", data = "<data>")]
pub fn set_evaluation_date(
    db: DbConn,
    logger: State<Logger>,
    data: Json<SetEvaluationDate>,
) -> Result<JsonValue> {
    KeyStore::set_evaluation_date(&db, &logger, data.evaluation_date.clone())?;
    Ok(json!({"done": true}))
}
