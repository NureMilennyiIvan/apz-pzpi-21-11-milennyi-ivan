use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
     // Paths corresponding to various service methods across different modules are included here
    paths(
    ),
     // Components such as schemas used in API documentation are defined here
    components(
        schemas(
        )
    ),
    // Tags for grouping API endpoints are defined here
    tags(
        (name = "Server", description = "API view")
    )
)]
pub(crate) struct ApiDoc;