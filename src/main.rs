#[macro_use]
extern crate rocket;

pub mod file_op;
pub mod log_op;

use std::net::Ipv4Addr;
use file_op::read_lines;
use log_op::Log;
use rocket::{serde::json::{Json, from_str}, Build, Config, Rocket, Route};

const LOG_FILE_PATH: &str = "./logs/logs";

#[get("/logchest/logs")]
fn display_logs() -> Json<Vec<Log>> {
    let transformer = |line: (usize, &str)| -> Log {
        let log = from_str::<Log>(line.1).unwrap();

        log.read_nomalizing()
    };
    Json(
        match read_lines::<Log>(LOG_FILE_PATH, transformer) {
            Ok(res) => res,
            Err(_) => vec!(),
        }
    )
    
}

#[post("/logchest/log", format = "json", data = "<log>")]
fn add_log(mut log: Json<Log>) -> String {
    let mut fm8kr = file_op::create_file(LOG_FILE_PATH);
    if let Err(res) = fm8kr.with_directories().go_my_dude() {
        return res.to_string();
    } 

    match (*log).push_log() {
        Ok(msg) => {
            if let Err(err) = fm8kr.write_line(&msg) {
                return err.to_string();
            }
            format!("log added msg: {}, date: {}", log.msg, log.date)
        }
        Err(err) => err.to_string(),
    }
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
    lezgong(routes![display_logs, add_log], 8081)
}
