use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::db::cycles::Cycle;
use crate::db::general_comment::{EvalComment, GeneralComment};
use crate::db_log::Logger;
use crate::{DbConn, Result};

#[get("/comment/<cycle>")]
pub fn comment(db: DbConn, cycle: String) -> Result<JsonValue> {
    Ok(json!({"comment": EvalComment::comment(&db, &Cycle::from_text(&cycle))?}))
}

#[derive(Deserialize)]
pub struct UpdateComment {
    comment: String,
}

#[put("/comment/<id>", format = "json", data = "<data>")]
pub fn update(
    db: DbConn,
    logger: State<Logger>,
    id: i32,
    data: Json<UpdateComment>,
) -> Result<JsonValue> {
    GeneralComment::update(&db, &logger, id, Some(data.comment.clone()))?;
    Ok(json!({"done": true}))
}
