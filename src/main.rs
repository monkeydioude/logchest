#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::response::content;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Log {
    #[serde(default)]
    id: i64,
    level: i32,
    msg: String,
}

#[get("/logs")]
fn display_logs() -> Json<Vec<Log>> {
    let file = std::fs::File::open("logs/logs");
    // LogData("[\"salut les kids\"]")
    Json(vec![Log{
        id: 0,
        level: 0,
        msg: "Ya rukzak".to_string(),
    }])
}

#[post("/log", format = "json", data = "<log>")]
fn add_log(log: Json<Log>) -> String {
    
    format!("log added id:{}, level:{}, msg: {}", log.id, log.level, log.msg)
}

#[launch]
fn lezgong() -> _ {
    rocket::build()
        .mount("/", routes![display_logs, add_log])
}
