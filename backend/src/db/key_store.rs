use crate::db_log::Logger;
use crate::Result;
use chrono::NaiveDate;
use diesel::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Serialize;

const DEFAULT_EVALUATION_DATE: &'static str = "2020-12-31";

#[derive(Queryable, Serialize)]
pub struct KeyStore {
    pub name: String,
    pub value: Option<String>,
}

impl KeyStore {
    pub fn get_evaluation_date(db: &PgConnection) -> Result<NaiveDate> {
        use crate::schema::key_store;
        use crate::schema::key_store::dsl;
        let value =
            if let Ok(key_store) = dsl::key_store.find("evaluation_date").first::<KeyStore>(db) {
                key_store.value.unwrap()
            } else {
                // Insert
                diesel::insert_into(dsl::key_store)
                    .values((
                        key_store::name.eq("evaluation_date".to_string()),
                        key_store::value.eq(DEFAULT_EVALUATION_DATE.to_string()),
                    ))
                    .execute(db)?;
                // And try again
                let key_store = dsl::key_store
                    .find("evaluation_date")
                    .first::<KeyStore>(db)?;
                key_store.value.unwrap()
            };
        Ok(NaiveDate::parse_from_str(&value, "%Y-%m-%d")?)
    }

    pub fn set_evaluation_date(
        db: &PgConnection,
        log: &Logger,
        evaluation_date: NaiveDate,
    ) -> Result<()> {
        use crate::schema::key_store;
        use crate::schema::key_store::dsl;
        diesel::update(dsl::key_store.find("evaluation_date".to_string()))
            .set(key_store::value.eq(evaluation_date.to_string()))
            .execute(db)?;
        log.update1(
            "key_store".to_string(),
            0,
            "evaluation_date".to_string(),
            evaluation_date.to_string(),
        )?;
        Ok(())
    }
}
