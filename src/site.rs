use std::thread;
pub use rocket::{Rocket,response,request::FromRequest};
pub use rocket::http;
pub use rocket_contrib::templates::Template;

pub mod index;
pub mod chat;
pub mod utils;


pub fn launch() {
    let r = thread::spawn(|| {
        let r = rocket::ignite()
        .mount("/page",rocket_contrib::serve::StaticFiles::from("page"))
        .mount("/",rocket::routes![index::index,chat::chat])
        .attach(Template::fairing())
        .launch();
    }).join().unwrap();
}