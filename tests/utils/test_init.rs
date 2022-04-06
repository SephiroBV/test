use actix_web::web;

use test::init::app_config;

pub fn test_config(cfg: &mut web::ServiceConfig) {
    app_config::config(cfg);
}
