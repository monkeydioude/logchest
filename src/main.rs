#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

use rocket::{serde::json::Json, Config, Route, Build, Rocket};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Log {
    #[serde(default)]
    id: i64,
    level: i32,
    msg: String,
}

#[get("/logs")]
fn display_logs() -> Json<Vec<Log>> {
    // LogData("[\"salut les kids\"]")
    Json(vec![Log {
        id: 0,
        level: 0,
        msg: "Ya rukzak".to_string(),
    }])
}

#[post("/log", format = "json", data = "<log>")]
fn add_log(log: Json<Log>) -> String {
    format!(
        "log added id:{}, level:{}, msg: {}",
        log.id, log.level, log.msg
    )
}

fn launch(routes: Vec<Route>, port: u16) -> Rocket<Build> {
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
fn lezgong() -> _ {
    launch(routes![display_logs, add_log], 8080)
}
