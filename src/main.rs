#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

#[get("/motd")]
fn motd() -> Template {
    Template::render("components/motd", context! {
        motd: format!("Hello, world! You are the {} visitor.", to_positional(fetch_add_counter())),
    })
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

fn to_positional(num: usize) -> String {
    if num % 100 == 11 || num % 100 == 12 || num % 100 == 13 {
        return format!("{}th", num);
    }

    match num % 10 {
        1 => format!("{}st", num),
        2 => format!("{}nd", num),
        3 => format!("{}rd", num),
        _ => format!("{}th", num),
    }
}

// global counter
static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
fn fetch_add_counter() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
}

#[get("/example")]
fn example() -> Template {
    
    // COUNTER.
    Template::render("example", context! {
        name: "Rocket", 
        items: vec![
            "Boots".to_string(),
            "Dookie".to_string(),
            "Buttons".to_string(),]
    })
}

#[get("/template")]
fn template() -> Template {
    Template::render("base", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static/", FileServer::from(relative!("static")))
        .mount("/", routes![index, example, template, motd])
        .attach(Template::fairing())
}