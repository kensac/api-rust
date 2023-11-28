# API-RUST

Note: Every time you run a prisma command, go to the autogenerated prisma.rs file in src and add #[derive(Clone)] to the struct on top of PrismaClient. This is because the prisma.rs file is autogenerated and will be overwritten every time you run a prisma command and by default does not implement the Clone trait. This is needed for the API to work.

# To run the API

1. Install Rust from https://www.rust-lang.org/tools/install
2. Run the following commands:
   `cargo run --package api-rust --bin api-rust`

# Tasks

- [] AT THE END: Recheck all the docs to ensure they are compatible with the new structs
- [] AT THE END: Check if the app_state is being called in a function incorrectly
- [] update the return types of all the functions to be extra descriptive
- [] Adding to the point above this, most functions should have a generic return type that can belike pub type BaseReturnType<T> = Result<(StatusCode, T), (StatusCode, String)>;. However since the developer will only see BaseReturnType<T> and not the actual return type, it takes away from the readability and verbosity of the code. So we need to find a way to make the return types more descriptive while also being generic
- [] add auth. Current alternative solution is to mostly employ a similar method as we saw in v2. However instead of having the Roles be stored on firebase, we just use the firebase API endpoint to verify the user when we get the Bearer token from the user and then have the permissions stored on our database. This would mean we can then then verify the necessary permission with our database instead of offloading to firebase. this would make customUserClaims redundant. The only question would be is it sufficient to check the UID to verify that we have the right user. Since firebase says the uid is unique I am assuming that propagates to our service as well?
- [] add tests. Issues when I tried to implement testing: One, we need a mock server to be able to do tests. We need to figure out
- [] add logging. Update, we have logging right now that works using a tracing layer on the API. It is currently set to only output ERROR level logs. We can change this to be more verbose if we want to. This will be changed when we deploy but it was just becoming very annoying to Debug the API with verbose logs.
- [] make all structs camelCase based on https://jsonapi.org/recommendations/#:~:text=Member%20names%20SHOULD%20be%20camel,and%20%E2%80%9C0%2D9%E2%80%9D
- [] make sure time in chrono(fixedOffset) is compatible with https://www.w3.org/TR/NOTE-datetime otherwise add converter utility function. I believe it is compatible but need to double check. It might also hel to have it convert from a simpler format to the fixedOffset format because it is easier to read and write. The fixedOffset format while precise is way too detailed for our use case. However, it is a Prisma default so we will have to figure out how to change it.
- [] reformat path params to be struct so that we get serde deserialization and get better parsing errors especially with invalid data types

# Notes about cool axum/web dev stuff that I found out

- if the path parameter doesn't return the required entity, axum will return a 404 error automatically (ps I forgot what was supposed to be here and saw the blank and filled in what I thought is correct)