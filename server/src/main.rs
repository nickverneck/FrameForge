use rocket::{get, routes, launch};
use rocket::serde::{Serialize, json::Json};
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealthStatus{
    status: String,
    uptime_seconds: u64,
}

#[get("/")]
fn index() -> &'static str {
    "Hello , World"
}
#[get("/health")]
fn health()-> Json<HealthStatus> {

    Json(HealthStatus{
        status:"ok".to_string(),
        uptime_seconds:0,

    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, health])

}
