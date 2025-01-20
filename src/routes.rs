use actix_web::web::ServiceConfig;

mod authentification;
mod create_data;
mod read_data;

// Function to configure all routes
pub fn v1_routes(cfg: &mut ServiceConfig) {
    cfg.service(authentification::sign_up)
        .service(authentification::sign_in)
        .service(read_data::read_user_metadata)
        .service(create_data::create_tags);
}
