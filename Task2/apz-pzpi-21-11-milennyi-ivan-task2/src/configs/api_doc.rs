use utoipa::OpenApi;
use crate::models::*;
use crate::view_models::*;
use crate::view_models::extra_view_models::*;
use crate::json_structs::*;
use crate::endpoints::{
    breed_endpoints::*,
    feed_endpoints::*,
    feed_supply_endpoints::*,
    feeding_log_endpoints::*,
    shearing_log_endpoints::*,
    sheep_endpoints::*,
    shepherd_endpoints::*,
    storekeeper_endpoints::*,
    temperature_scanner_endpoints::*
};
#[derive(OpenApi)]
#[openapi(
     // Paths corresponding to various service methods across different modules are included here
    paths(
        breed_get_all,
        breed_by_id,
        breed_all_vms,
        breed_create,
        breed_update,
        breed_delete,
        feed_get_all,
        feed_by_id,
        feed_all_vms,
        feed_create,
        feed_update,
        feed_delete,
        feed_supply_get_all,
        feed_supply_by_id,
        feed_supply_all_vms,
        feed_supply_create,
        feed_supply_delete,
        feeding_log_create,
        feeding_log_delete,
        feeding_log_get_all,
        feeding_log_get_by_id,
        feeding_log_get_all_vms_by_sheep_id,
        feeding_log_get_all_vms_by_feed_id,
        shearing_log_create,
        shearing_log_delete,
        shearing_log_get_all,
        shearing_log_get_by_id,
        shearing_log_get_all_vms_by_sheep_id,
        sheep_get_all,
        sheep_get_by_id,
        sheep_get_details_by_id,
        sheep_get_all_vms_by_shepherd_id,
        sheep_create,
        sheep_update,
        sheep_change_shepherd,
        sheep_delete,
    ),
     // Components such as schemas used in API documentation are defined here
    components(
        schemas(
            Breed, Feed, FeedSupply, FeedingLog, ShearingLog, Sheep, Shepherd, Storekeeper, TemperatureScanner,
            BreedVM, FeedVM, FeedSupplyVM, FeedingLogVM, ShearingLogVM, SheepVM, ShepherdVM, StorekeeperVM, TemperatureScannerVM,
            SheepDetailsVM, ChangeShepherdJson, PathId
        )
    ),
    // Tags for grouping API endpoints are defined here
    tags(
        (name = "Server", description = "API view")
    )
)]
pub(crate) struct ApiDoc;