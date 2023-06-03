#[macro_use]
extern crate rocket;

pub mod file_op;

use std::{net::Ipv4Addr};
use file_op::read_lines;
use rocket::{serde::json::Json, Build, Config, Rocket, Route};
use serde::{Deserialize, Serialize};

const LOG_FILE_PATH: &str = "./logs/logs";

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Log {
    #[serde(default)]
    id: i64,
    msg: String,
}


#[get("/logs")]
// fn display_logs() -> Json<Vec<Log>> {
fn display_logs() -> Json<Vec<String>> {
    Json(
        match read_lines(LOG_FILE_PATH, |line| line.1.to_string()) {
            Ok(res) => res,
            Err(_) => vec!(),
        }
    )
    
}

#[post("/log", format = "json", data = "<log>")]
fn add_log(log: Json<Log>) -> String {
    let mut fm8kr = file_op::create_file(LOG_FILE_PATH);
    if let Err(res) = fm8kr.with_directories().go_my_dude() {
        panic!("{}", res);
    }

    match fm8kr.write_line(&log.msg) {
        Ok(_) => format!(
            "log added id:{}, msg: {}",
            log.id, log.msg
        ),
        Err(err) => err.to_string(),
    }
    // println!("{:?}", &fm8kr);
    // fm8kr.with_directories();    
}

fn lezgong(routes: Vec<Route>, port: u16) -> Rocket<Build> {
    rocket::build()
        .configure(Config {
            port,
            address: match "0.0.0.0".parse::<Ipv4Addr>() {
                Ok(addr) => std::net::IpAddr::V4(addr),
                Err(_) => std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            },
            ..Config::default()
        })
        .mount("/", routes)
}

#[launch]
fn launch() -> _ {
    lezgong(routes![display_logs, add_log], 8080)
}
