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
// 8. This brings the Html type into scope,
use rocket_contrib::json::Json;

#[derive(Serialize)]