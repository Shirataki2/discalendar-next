mod list;
use crate::prelude::*;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/guilds")
            //.route("/authorize", web::get().to(oauth2::authorize))
            .service(list::list)
    );
}
