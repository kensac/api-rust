
use crate::prisma::{user, PrismaClient};

pub async fn create_user(client: PrismaClient, first_name: String, last_name: String) -> user::Data {
    let user: user::Data = client.user().create(first_name, last_name, vec![]).exec().await.unwrap();

    return user
}

pub async fn get_first_user(client: PrismaClient) -> user::Data {
    let user: user::Data = client.user().find_first(vec![]).exec().await.unwrap().unwrap();

    return user
}