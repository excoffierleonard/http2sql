use actix_web::web::ServiceConfig;

mod authentification;
mod user;

// Function to configure all routes
pub fn v1_routes(cfg: &mut ServiceConfig) {
    cfg.service(authentification::sign_up)
        .service(authentification::sign_in)
        .service(user::get_user_metadata);
}
