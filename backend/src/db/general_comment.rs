use crate::db::cycles::Cycle;
use crate::db::eleves::Eleve;
use crate::db_log::Logger;
use crate::Result;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct GeneralComment {
    pub id: i32,
    pub eleve_id: i32,
    pub comment: Option<String>,
}

impl GeneralComment {
    pub fn by_eleve(db: &PgConnection, eleve_id: i32) -> Result<GeneralComment> {
        use crate::schema::general_comments;
        use crate::schema::general_comments::dsl;
        if let Ok(g) = dsl::general_comments
            .filter(general_comments::eleve_id.eq(eleve_id))
            .first::<GeneralComment>(db)
        {
            Ok(g)
        } else {
            // Insert a new one
            diesel::insert_into(dsl::general_comments)
                .values((
                    general_comments::eleve_id.eq(eleve_id),
                    general_comments::comment.eq("".to_string()),
                ))
                .execute(db)?;
            // And try again !
            Ok(dsl::general_comments
                .filter(general_comments::eleve_id.eq(eleve_id))
                .first::<GeneralComment>(db)?)
        }
    }

    /// Return id of the new GeneralComment
    pub fn new(
        db: &PgConnection,
        log: &Logger,
        eleve_id: i32,
        comment: Option<String>,
    ) -> Result<i32> {
        use crate::schema::general_comments;
        use crate::schema::general_comments::dsl;
        let id = diesel::insert_into(dsl::general_comments)
            .values((
                general_comments::eleve_id.eq(eleve_id),
                general_comments::comment.eq(comment.clone()),
            ))
            .returning(general_comments::id)
            .get_result(db)?;
        log.insert2(
            "general_comments".to_string(),
            "eleve_id".to_string(),
            eleve_id.to_string(),
            "comment".to_string(),
            comment.unwrap_or("".to_string()),
        )?;
        Ok(id)
    }

    pub fn update(db: &PgConnection, log: &Logger, id: i32, comment: Option<String>) -> Result<()> {
        use crate::schema::general_comments;
        use crate::schema::general_comments::dsl;
        diesel::update(dsl::general_comments.find(id))
            .set(general_comments::comment.eq(comment.clone()))
            .execute(db)?;
        log.update1(
            "general_comments".to_string(),
            id,
            "comment".to_string(),
            comment.unwrap_or("".to_string()),
        )?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct EvalComment {
    pub id: i32,
    pub eleve_firstname: String,
    pub eleve_lastname: String,
    pub comment: Option<String>,
}

impl EvalComment {
    pub fn comment(db: &PgConnection, cycle: &Cycle) -> Result<Vec<EvalComment>> {
        let mut comments = vec![];
        let eleves = Eleve::by_cycle(db, cycle)?;
        for eleve in eleves.iter() {
            let comment = GeneralComment::by_eleve(db, eleve.id)?;
            comments.push(EvalComment {
                id: comment.id,
                eleve_firstname: eleve.firstname.clone(),
                eleve_lastname: eleve.lastname.clone(),
                comment: comment.comment.clone(),
            });
        }
        Ok(comments)
    }
}
