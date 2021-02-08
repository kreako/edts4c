#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod db;
pub mod db_log;
pub mod error;
pub mod routes;
pub mod schema;

use diesel::PgConnection;
use rocket_contrib::database;
use rocket_contrib::serve::StaticFiles;

use std::result;

/// The result type of this program.
pub type Result<T> = result::Result<T, error::Error>;

#[database("postgres_database")]
pub struct DbConn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .manage(db_log::Logger::new())
        .mount(
            "/api/domains/",
            routes![
                routes::domains::all,
                routes::domains::set_title,
                routes::domains::set_domains_rank,
                routes::domains::append,
                routes::domains::delete,
            ],
        )
        .mount(
            "/api/components/",
            routes![
                routes::components::all,
                routes::components::all_ordered,
                routes::components::summary,
                routes::components::by_domain_id,
                routes::components::set_title,
                routes::components::set_components_rank,
                routes::components::append,
                routes::components::delete,
                routes::components::move_to_domain,
            ],
        )
        .mount(
            "/api/competencies/",
            routes![
                routes::competencies::all,
                routes::competencies::set_title,
                routes::competencies::by_component_id,
                routes::competencies::update_cycle_c1,
                routes::competencies::update_cycle_c2,
                routes::competencies::update_cycle_c3,
                routes::competencies::update_cycle_c4,
                routes::competencies::set_competencies_rank,
                routes::competencies::append,
                routes::competencies::delete,
                routes::competencies::move_to_component,
                routes::competencies::by_id,
            ],
        )
        .mount(
            "/api/eleves/",
            routes![
                routes::eleves::all,
                routes::eleves::by_id,
                routes::eleves::by_cycle,
                routes::eleves::new,
                routes::eleves::update,
                routes::eleves::set_active,
            ],
        )
        .mount(
            "/api/evaluations/",
            routes![
                routes::evaluations::by_id,
                routes::evaluations::by_eleve,
                routes::evaluations::stats,
                routes::evaluations::by_cycle_and_competency,
                routes::evaluations::set_status,
                routes::evaluations::set_comment,
                routes::evaluations::toc,
                routes::evaluations::detail,
            ],
        )
        .mount(
            "/api/key_store/",
            routes![
                routes::key_store::evaluation_date,
                routes::key_store::set_evaluation_date,
            ],
        )
        .mount(
            "/api/report/",
            routes![routes::report::summary, routes::report::full,],
        )
        .mount(
            "/api/general_comments/",
            routes![
                routes::general_comment::comment,
                routes::general_comment::update,
            ],
        )
        .mount(
            "/api/socle/",
            routes![routes::socle::toc, routes::socle::component,],
        )
        .mount("/", StaticFiles::from("frontend"))
        .launch();
    // println!("{}: {}", file!(), line!());
}
