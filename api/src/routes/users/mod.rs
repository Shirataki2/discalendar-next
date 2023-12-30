use actix_web::web;
mod me;


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            //.route("/authorize", web::get().to(oauth2::authorize))
            .service(me::me)
    );
}
