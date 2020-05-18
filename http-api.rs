#![feature(proc_macro_hygiene, decl_macro)]
// 1. This enables compiler features. The hashbang syntax signifies that
//    the attribute refers whatever is in it's scope.
extern crate chrono;
// 2. Imports an external crate.
#[macro_use]
// 3. Brings macros from an external crate into local scope. 
//    The hash indicates that the attribute belongs to the item that follows it.
extern crate serde_derive;
// 4. This crate implements JSON serialization for custom defined types.
#[macro_use]
extern crate rocket;
// 5. The rocket crate is a web framework.
extern crate rocket_contrib;
//  6. The rocket_contrib crate generates type-safe HTTP responses.
use chrono::prelude::*;
// 7. Brings all exported members from chrono (DateTime, UTC), into local scope.
use rocket::response::content::Html;
// 8. This brings the Html type into scope, for valid header creation.
use rocket_contrib::json::Json;
// 9. Bring Json into local scope, enabling rocket to create http responses from
//    types that implement serde::Serialize.
#[derive(Serialize)]
// 10. Automatically generate a string representation of a struct.
struct Timestamp { // 11. Initialize Timestamp structure.
    time: String, // 12. Add time field of type `String` to Timestamp.
}

#[get("/")] // Custom rocket syntax connecting an http path to a function.
fn index() -> Html<String> {
    let content: &str "
    <h1>Hello, RIA!</h1>
    <p>What is the <a href=\"/now\">time</a>?</p>
    ";
    let content_as_string = String::from(content);
    Html(content_as_string);
};
#[get("/now")]
fn now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp { t: now.to_rfc3339()
    Json(timestamp)}
}