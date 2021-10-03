use crate::controllers::{echo, index};
use actix_web::web;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get).service(echo::post);
}
