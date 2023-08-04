#[macro_use]
extern crate rocket;

pub mod file_op;
pub mod log_op;

use std::{net::Ipv4Addr, time::{UNIX_EPOCH, SystemTime}};
use file_op::read_lines;
use log_op::Log;
use rocket::{serde::json::{Json, to_string}, Build, Config, Rocket, Route};

const LOG_FILE_PATH: &str = "./logs/logs";

#[get("/logs")]
fn display_logs() -> Json<Vec<String>> {
    Json(
        match read_lines(LOG_FILE_PATH, |line| line.1.to_string()) {
            Ok(res) => res,
            Err(_) => vec!(),
        }
    )
    
}

#[post("/log", format = "json", data = "<log>")]
fn add_log(mut log: Json<Log>) -> String {
    let mut fm8kr = file_op::create_file(LOG_FILE_PATH);
    if let Err(res) = fm8kr.with_directories().go_my_dude() {
        panic!("{}", res);
    }
    log.date = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match to_string(&*log) {
            Ok(msg) => {
                if let Err(err) = fm8kr.write_line(&msg) {
                    return err.to_string()
                }
                format!(
                    "log added msg: {}, date: {}",
                    log.msg, log.date
                )
            },
            Err(err) => err.to_string()
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
    lezgong(routes![display_logs, add_log], 8080)
}
