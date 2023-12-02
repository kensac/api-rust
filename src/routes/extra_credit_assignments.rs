use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

struct ExtraCreditAssignmentsEntity {
    user_id: Uuid,
    class_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
struct CreateExtraCreditAssignmentEntity {
    user_id: Uuid,
    class_id: Uuid,
}

/*
Current implementation doesn't make sense. What are the users that go into extra credit classes. And are assignments just based on which event a user checks into? This probably needs to be rethought and reworked.
*/
