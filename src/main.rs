#[macro_use]
extern crate rocket;

use nanoid::nanoid;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::{catchers, State}; // needed to store state across the application
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct UrlInformation {
    identifier: String,
    full_url: String,
    creator: String,
    visible: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct WelcomeMessage {
    message: String,
    version: String,
    total_urls: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UrlMetadata {
    url: String,
    creator: String,
}

// this way is not the best and more performant way but it's fast and easy to implement
type InMemoryCache = Mutex<HashMap<String, UrlInformation>>;

#[post("/new", format = "application/json", data = "<url>")]
async fn new_url(records: &State<InMemoryCache>, url: Json<UrlMetadata>) -> Value {
    let metadata = url.into_inner();

    // generate nano id
    let new_identifier = nanoid!();

    // lock list to make it mutable using mutex
    let mut list = records.lock().await;

    list.insert(
        new_identifier.clone(),
        UrlInformation {
            creator: metadata.creator,
            full_url: metadata.url,
            identifier: new_identifier.clone(),
            visible: true,
        },
    );

    json!({
        "status" : "200",
        "message" : "Record inserted",
        "identifier" : new_identifier
    })
}

#[get("/")]
async fn index(records: &State<InMemoryCache>) -> Json<WelcomeMessage> {
    let lock_records = records.lock().await;
    Json(WelcomeMessage {
        message: String::from("Welcome to Rustener, a fast url shortener"),
        version: "0.0.1".to_string(),
        total_urls: lock_records.len(),
    })
}

#[get("/<identifier>")]
async fn get_url(identifier: String, records: &State<InMemoryCache>) -> Option<Value> {
    let lock_records = records.lock().await;
    let url_information: &UrlInformation = lock_records.get(&identifier)?;

    Some(json!(*url_information))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "404",
        "reason": "Resource not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(InMemoryCache::new(HashMap::new()))
        .mount("/", routes![index, new_url, get_url])
        .register("/", catchers![not_found])
}
