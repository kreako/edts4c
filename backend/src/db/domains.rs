use crate::db_log::Logger;
use crate::Result;
use diesel::dsl::max;
use diesel::PgConnection;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize, Clone)]
pub struct Domain {
    pub id: i32,
    pub rank: i32,
    pub title: String,
}

impl Domain {
    pub fn all(db: &PgConnection) -> Result<Vec<Domain>> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        let domains = dsl::domains.order(domains::rank).load::<Domain>(db);
        Ok(domains?)
    }

    pub fn by_id(db: &PgConnection, id: i32) -> Result<Domain> {
        use crate::schema::domains::dsl;
        let domain = dsl::domains.find(id).get_result::<Domain>(db)?;
        Ok(domain)
    }

    pub fn next_domain_id(db: &PgConnection, rank: i32) -> Result<Option<i32>> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        let id = dsl::domains
            .select(domains::id)
            .order(domains::rank)
            .filter(domains::rank.gt(rank))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    pub fn previous_domain_id(db: &PgConnection, rank: i32) -> Result<Option<i32>> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        let id = dsl::domains
            .select(domains::id)
            .order(domains::rank.desc())
            .filter(domains::rank.lt(rank))
            .first::<i32>(db)
            .optional()?;
        Ok(id)
    }

    pub fn set_title(db: &PgConnection, log: &Logger, id: i32, title: String) -> Result<()> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        diesel::update(dsl::domains.find(id))
            .set(domains::title.eq(title.clone()))
            .execute(db)?;
        log.update1("domains".to_string(), id, "title".to_string(), title)?;
        Ok(())
    }

    pub fn set_rank(db: &PgConnection, id: i32, rank: i32) -> Result<()> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        diesel::update(dsl::domains.find(id))
            .set(domains::rank.eq(rank))
            .execute(db)?;
        Ok(())
    }

    pub fn set_domains_rank(db: &PgConnection, domains: Vec<Domain>) -> Result<()> {
        // domains array arrives sorted as the user wants it
        // with invalid rank inside that I need to set
        let mut rank = 1;
        for domain in domains {
            Domain::set_rank(db, domain.id, rank)?;
            rank += 1;
        }
        Ok(())
    }

    /// Return the new Domain
    pub fn append(db: &PgConnection, title: String) -> Result<Domain> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        // Select the current max rank
        let max_rank: Option<i32> = dsl::domains.select(max(domains::rank)).first(db)?;
        let rank: i32 = max_rank.unwrap_or(0) + 1;
        // insert
        let domain_id: i32 = diesel::insert_into(dsl::domains)
            .values((domains::rank.eq(rank), domains::title.eq(title.clone())))
            .returning(domains::id)
            .get_result(db)?;
        // return the new one
        Domain::by_id(db, domain_id)
    }

    pub fn delete(db: &PgConnection, id: i32) -> Result<()> {
        use crate::schema::domains;
        use crate::schema::domains::dsl;
        diesel::delete(dsl::domains.filter(domains::id.eq(id))).execute(db)?;
        Domain::fix_rank(db)?;
        Ok(())
    }

    /// Fix rank when something is moved or deleted
    pub fn fix_rank(db: &PgConnection) -> Result<()> {
        let domains = Domain::all(db)?;
        Domain::set_domains_rank(db, domains)?;
        Ok(())
    }
}
