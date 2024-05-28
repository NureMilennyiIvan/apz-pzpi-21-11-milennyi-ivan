use utoipa::OpenApi;
use crate::models::*;
use crate::view_models::*;
use crate::view_models::extra_view_models::*;
use crate::json_structs::*;
use crate::endpoints::*;
#[derive(OpenApi)]
#[openapi(
     // Paths corresponding to various service methods across different modules are included here
    paths(
        sheep_endpoints::sheep_get_all,
        sheep_endpoints::sheep_get_by_id,
        sheep_endpoints::sheep_get_details_by_id,
        sheep_endpoints::sheep_get_all_vms,
        sheep_endpoints::sheep_create,
        sheep_endpoints::sheep_update,
        sheep_endpoints::sheep_change_shepherd,
        sheep_endpoints::sheep_delete,
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