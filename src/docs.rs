use utoipa::OpenApi;

use crate::routes;
use crate::utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        utils::health_check,
        routes::hackathons::create_hackathon,
        routes::hackathons::get_hackathon,
        routes::hackathons::get_hackathon_by_id,
        routes::hackathons::delete_hackathon_by_id,
        routes::hackathons::set_active_hackathon,
        routes::extra_credit_classes::create_extra_credit_class,
        routes::extra_credit_classes::get_all_extra_credit_classes,
        routes::extra_credit_classes::get_extra_credit_class_by_id,
        routes::extra_credit_classes::delete_extra_credit_class_by_id,
        routes::locations::create_location,
        routes::locations::get_all_locations,
        routes::locations::get_location_by_id,
        routes::locations::delete_location_by_id,
    ),
    components(schemas(
        routes::hackathons::CreateHackathonEntity,
        routes::hackathons::HackathonEntity,
        routes::extra_credit_classes::CreateExtraCreditClassEntity,
        routes::locations::CreateLocationEntity,
    ))
)]
pub struct ApiDoc;
