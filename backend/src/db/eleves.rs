use crate::db::cycles::Cycle;
use crate::db::evaluations::Evaluation;
use crate::db::general_comment::GeneralComment;
use crate::db::key_store::KeyStore;
use crate::db_log::Logger;
use crate::Result;
use chrono::{Datelike, Duration, NaiveDate};
use diesel::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Eleve {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthdate: NaiveDate,
    pub school_entry: NaiveDate,
    pub cycle: String,
    pub active: bool,
}

impl Eleve {
    /// Return eleves, only the active ones
    pub fn all(db: &PgConnection) -> Result<Vec<Eleve>> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        let eleves = dsl::eleves
            .filter(eleves::active.eq(true))
            .order(eleves::firstname)
            .load::<Eleve>(db);
        Ok(eleves?)
    }

    /// Return the list of all eleve id
    pub fn all_id(db: &PgConnection) -> Result<Vec<i32>> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        Ok(dsl::eleves.select(eleves::id).load::<i32>(db)?)
    }

    /// Return eleves, only the active ones from a cycle
    pub fn by_cycle(db: &PgConnection, cycle: &Cycle) -> Result<Vec<Eleve>> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        let eleves = dsl::eleves
            .filter(eleves::active.eq(true))
            .filter(eleves::cycle.eq(cycle.to_string()))
            .order(eleves::firstname)
            .load::<Eleve>(db);
        Ok(eleves?)
    }

    /// Return one eleve by id
    pub fn by_id(db: &PgConnection, id: i32) -> Result<Eleve> {
        use crate::schema::eleves::dsl;
        let eleve = dsl::eleves.find(id).first::<Eleve>(db);
        Ok(eleve?)
    }

    /// Return id of the new eleve
    pub fn new(
        db: &PgConnection,
        log: &Logger,
        firstname: String,
        lastname: String,
        birthdate: NaiveDate,
        school_entry: NaiveDate,
        cycle: Cycle,
    ) -> Result<i32> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        let eleve_id = diesel::insert_into(dsl::eleves)
            .values((
                eleves::firstname.eq(firstname.clone()),
                eleves::lastname.eq(lastname.clone()),
                eleves::birthdate.eq(birthdate.clone()),
                eleves::school_entry.eq(school_entry.clone()),
                eleves::cycle.eq(cycle.to_string()),
                eleves::active.eq(true),
            ))
            .returning(eleves::id)
            .get_result(db)?;
        log.insert5(
            "eleves".to_string(),
            "firstname".to_string(),
            firstname,
            "lastname".to_string(),
            lastname,
            "birthdate".to_string(),
            birthdate.to_string(),
            "school_entry".to_string(),
            school_entry.to_string(),
            "cycle".to_string(),
            cycle.to_string(),
        )?;
        Evaluation::create_empty_for_eleve(db, eleve_id)?;
        GeneralComment::new(db, log, eleve_id, None)?;
        Ok(eleve_id)
    }

    pub fn update(
        db: &PgConnection,
        log: &Logger,
        id: i32,
        firstname: String,
        lastname: String,
        birthdate: NaiveDate,
        school_entry: NaiveDate,
        cycle: Cycle,
    ) -> Result<()> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        diesel::update(dsl::eleves.find(id))
            .set((
                eleves::firstname.eq(firstname.clone()),
                eleves::lastname.eq(lastname.clone()),
                eleves::birthdate.eq(birthdate.clone()),
                eleves::school_entry.eq(school_entry.clone()),
                eleves::cycle.eq(cycle.to_string()),
            ))
            .execute(db)?;
        log.update5(
            "eleves".to_string(),
            id,
            "firstname".to_string(),
            firstname,
            "lastname".to_string(),
            lastname,
            "birthdate".to_string(),
            birthdate.to_string(),
            "school_entry".to_string(),
            school_entry.to_string(),
            "cycle".to_string(),
            cycle.to_string(),
        )?;
        Ok(())
    }

    pub fn set_active(db: &PgConnection, log: &Logger, id: i32, active: bool) -> Result<()> {
        use crate::schema::eleves;
        use crate::schema::eleves::dsl;
        diesel::update(dsl::eleves.find(id))
            .set(eleves::active.eq(active))
            .execute(db)?;
        log.update1(
            "eleves".to_string(),
            id,
            "active".to_string(),
            active.to_string(),
        )?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CycleEleve {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthdate: NaiveDate,
    pub school_entry: NaiveDate,
    pub cycle: String,
    pub estimated_cycle: String,
}

impl CycleEleve {
    pub fn all(db: &PgConnection) -> Result<Vec<CycleEleve>> {
        let mut cycle_eleves = vec![];
        let evaluation_date = KeyStore::get_evaluation_date(db)?;
        let eleves = Eleve::all(db)?;
        for eleve in eleves.iter() {
            cycle_eleves.push(CycleEleve {
                id: eleve.id,
                firstname: eleve.firstname.clone(),
                lastname: eleve.lastname.clone(),
                birthdate: eleve.birthdate.clone(),
                school_entry: eleve.school_entry.clone(),
                cycle: eleve.cycle.clone(),
                estimated_cycle: estimate_cycle(eleve.birthdate, evaluation_date).to_string(),
            });
        }
        Ok(cycle_eleves)
    }

    /// Return one eleve by id
    pub fn by_id(db: &PgConnection, id: i32) -> Result<CycleEleve> {
        let eleve = Eleve::by_id(db, id)?;
        let evaluation_date = KeyStore::get_evaluation_date(db)?;
        Ok(CycleEleve {
            id: eleve.id,
            firstname: eleve.firstname.clone(),
            lastname: eleve.lastname.clone(),
            birthdate: eleve.birthdate.clone(),
            school_entry: eleve.school_entry.clone(),
            cycle: eleve.cycle.clone(),
            estimated_cycle: estimate_cycle(eleve.birthdate, evaluation_date).to_string(),
        })
    }
}

lazy_static! {
    static ref YEARS_6: Duration = Duration::days(6 * 365 + 1);
    static ref YEARS_9: Duration = Duration::days(9 * 365 + 2);
    static ref YEARS_12: Duration = Duration::days(12 * 365 + 3);
}

fn estimate_cycle(birthdate: NaiveDate, evaluation_date: NaiveDate) -> Cycle {
    // First estimate scholar year of the evaluation
    let scholar_year = if evaluation_date.month() > 8 {
        evaluation_date.year()
    } else {
        evaluation_date.year() - 1
    };
    // the date corresponding to the end of the year in the scholar year
    let end_of_year = NaiveDate::from_ymd(scholar_year, 12, 31);
    let age = end_of_year - birthdate;
    if age < *YEARS_6 {
        Cycle::C1
    } else if age < *YEARS_9 {
        Cycle::C2
    } else if age < *YEARS_12 {
        Cycle::C3
    } else {
        Cycle::C4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_estimate_cycle_c1_1() {
        let birthdate = NaiveDate::from_ymd(2015, 11, 2);
        let evaluation_date = NaiveDate::from_ymd(2020, 12, 31);
        assert_eq!(estimate_cycle(birthdate, evaluation_date), Cycle::C1);
    }

    #[test]
    fn test_estimate_cycle_c1_2() {
        let birthdate = NaiveDate::from_ymd(2015, 11, 2);
        let evaluation_date = NaiveDate::from_ymd(2021, 06, 30);
        assert_eq!(estimate_cycle(birthdate, evaluation_date), Cycle::C1);
    }

    #[test]
    fn test_estimate_cycle_c2_1() {
        let birthdate = NaiveDate::from_ymd(2015, 11, 2);
        let evaluation_date = NaiveDate::from_ymd(2021, 12, 31);
        assert_eq!(estimate_cycle(birthdate, evaluation_date), Cycle::C2);
    }

    #[test]
    fn test_estimate_cycle_c2_2() {
        let birthdate = NaiveDate::from_ymd(2015, 11, 2);
        let evaluation_date = NaiveDate::from_ymd(2022, 06, 30);
        assert_eq!(estimate_cycle(birthdate, evaluation_date), Cycle::C2);
    }
}
