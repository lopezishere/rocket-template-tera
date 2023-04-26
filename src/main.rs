extern crate rocket;
extern crate rocket_dyn_templates;

use rocket::{fs::FileServer, get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { title: "Prueba".to_string() })
}
