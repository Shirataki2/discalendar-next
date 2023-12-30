use actix_web::web;

pub mod callback;
pub mod login;
pub mod logout;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            //.route("/authorize", web::get().to(oauth2::authorize))
            .service(callback::callback)
            .service(login::login)
            .service(logout::logout)
    );
}