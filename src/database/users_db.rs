use crate::prisma::{
    user::{self, Data, UniqueWhereParam},
    PrismaClient,
};

pub async fn create_user(
    client: PrismaClient,
    first_name: String,
    last_name: String,
) -> user::Data {
    let user: user::Data = client
        .user()
        .create(first_name, last_name, vec![])
        .exec()
        .await
        .unwrap();

    return user;
}

pub async fn get_first_user(client: PrismaClient) -> user::Data {
    let user: user::Data = client
        .user()
        .find_first(vec![])
        .exec()
        .await
        .unwrap()
        .unwrap();

    return user;
}

pub async fn get_user_by_id(client: PrismaClient, id: i32) -> user::Data {
    let user: user::Data = match client
        .user()
        .find_unique(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => panic!("User not found"),
        },
        Err(_) => panic!("User not found"),
    };
    return user;
}

pub async fn get_user_by_first_name(client: PrismaClient, first_name: String) -> user::Data {
    let user: Data = match client
        .user()
        .find_first(vec![user::first_name::equals(first_name)])
        .exec()
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => panic!("User not found"),
        },
        Err(_) => panic!("User not found"),
    };
    return user;
}
