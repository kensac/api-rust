use gcp_auth::{AuthenticationManager, CustomServiceAccount};

pub async fn create_jwt() -> String {
    let path = std::path::Path::new("./google-service-account.json");
    let service_account = CustomServiceAccount::from_file(path).unwrap();
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &["https://www.googleapis.com/auth/devstorage.full_control"];
    let token = authentication_manager.get_token(scopes).await.unwrap();
    token.as_str().to_string()
}
