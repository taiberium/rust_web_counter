#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicI64, Ordering};

use rocket::{Build, Rocket, State};

struct VisitorCounter {
    visitor: AtomicI64,
}

#[get("/")]
fn index(counter: &State<VisitorCounter>) -> String {
    counter.visitor.fetch_add(1, Ordering::Relaxed);
    return format!("counter: {}", counter.visitor.load(Ordering::Relaxed));
}


#[launch]
fn rocket() -> Rocket<Build> {
    let visitor_counter = VisitorCounter {
        visitor: AtomicI64::new(0)
    };
    rocket::build()
        .manage(visitor_counter)
        .mount("/", routes![index])
}