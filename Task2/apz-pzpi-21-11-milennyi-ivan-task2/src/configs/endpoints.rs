use actix_web::web::ServiceConfig;
use crate::endpoints::*;

pub(crate) fn breed_configure(cfg: &mut ServiceConfig){
    use breed_endpoints::*;
}
pub(crate) fn feed_configure(cfg: &mut ServiceConfig){
    use feed_endpoints::*;
}
pub(crate) fn feed_supply_configure(cfg: &mut ServiceConfig){
    use feed_supply_endpoints::*;
}
pub(crate) fn feeding_log_configure(cfg: &mut ServiceConfig){
    use feeding_log_endpoints::*;
}
pub(crate) fn shearing_log_configure(cfg: &mut ServiceConfig){
    use shearing_log_endpoints::*;
}
pub(crate) fn sheep_configure(cfg: &mut ServiceConfig) {
    use sheep_endpoints::*;

    cfg.service(sheep_create)
        .service(sheep_delete)
        .service(sheep_update)
        .service(sheep_change_shepherd)
        .service(sheep_get_all)
        .service(sheep_get_all_vms)
        .service(sheep_get_by_id)
        .service(sheep_get_details_by_id);
}
pub(crate) fn shepherd_configure(cfg: &mut ServiceConfig) {
    use shepherd_endpoints::*;
}
pub(crate) fn storekeeper_configure(cfg: &mut ServiceConfig) {
    use storekeeper_endpoints::*;
}
pub(crate) fn temperature_scanner_configure(cfg: &mut ServiceConfig) {
    use temperature_scanner_endpoints::*;
}
