use actix_web::web;

mod home;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(home::greet);
}
