use account_service::{
    AccountError, AccountService, EmailLink, LinkLoginPassword, LinkPhone,
    LoginFirstRegistration, PhoneFirstRegistration,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use directory_service::DirectoryService;
use persistence::InMemoryStore;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct AppState {
    pub accounts: AccountService,
    pub directory: DirectoryService,
}

impl AppState {
    pub fn new(store: InMemoryStore) -> Self {
        Self {
            accounts: AccountService::new(store.clone()),
            directory: DirectoryService::new(store),
        }
    }
}

pub fn build_router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/accounts/sign-up/phone", post(sign_up_phone))
        .route("/v1/accounts/sign-up/login", post(sign_up_login))
        .route("/v1/accounts/sign-in/bootstrap", post(sign_in_bootstrap))
        .route("/v1/accounts/link-email", post(link_email))
        .route("/v1/accounts/link-phone", post(link_phone))
        .route(
            "/v1/accounts/link-login-password",
            post(link_login_password),
        )
        .route("/v1/directory/username/lookup", get(username_lookup))
        .route(
            "/v1/directory/display-name/search",
            get(display_name_search),
        )
        .route("/v1/directory/phone/lookup", get(phone_lookup))
        .with_state(state)
}

pub fn build_router() -> Router {
    build_router_with_state(AppState::new(InMemoryStore::default()))
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn sign_up_phone(
    State(state): State<AppState>,
    Json(payload): Json<PhoneFirstRegistration>,
) -> Result<(StatusCode, Json<account_service::AccountRecord>), ApiError> {
    let account = state.accounts.register_phone_first(payload).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

async fn sign_up_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginFirstRegistration>,
) -> Result<(StatusCode, Json<account_service::AccountRecord>), ApiError> {
    let account = state.accounts.register_login_first(payload).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

#[derive(Debug, Deserialize)]
struct SignInBootstrapRequest {
    sign_in_id: String,
}

async fn sign_in_bootstrap(
    State(state): State<AppState>,
    Json(payload): Json<SignInBootstrapRequest>,
) -> Result<Json<account_service::SignInBootstrap>, ApiError> {
    let bootstrap = state.accounts.sign_in_bootstrap(&payload.sign_in_id).await?;
    Ok(Json(bootstrap))
}

async fn link_email(
    State(state): State<AppState>,
    Json(payload): Json<EmailLink>,
) -> Result<Json<account_service::RecoveryEmailView>, ApiError> {
    let linked = state.accounts.link_email(payload).await?;
    Ok(Json(linked))
}

async fn link_phone(
    State(state): State<AppState>,
    Json(payload): Json<LinkPhone>,
) -> Result<Json<account_service::AccountRecord>, ApiError> {
    let linked = state.accounts.link_phone(payload).await?;
    Ok(Json(linked))
}

async fn link_login_password(
    State(state): State<AppState>,
    Json(payload): Json<LinkLoginPassword>,
) -> Result<Json<account_service::AccountRecord>, ApiError> {
    let linked = state.accounts.link_login_password(payload).await?;
    Ok(Json(linked))
}

#[derive(Debug, Deserialize)]
struct LookupQuery {
    value: String,
}

async fn username_lookup(
    State(state): State<AppState>,
    Query(query): Query<LookupQuery>,
) -> Response {
    match state.directory.search_by_username(&query.value).await {
        Some(entry) => (StatusCode::OK, Json(entry)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Debug, Serialize)]
struct DisplayNameSearchResponse {
    results: Vec<directory_service::DisplayNameResult>,
}

async fn display_name_search(
    State(state): State<AppState>,
    Query(query): Query<LookupQuery>,
) -> Json<DisplayNameSearchResponse> {
    Json(DisplayNameSearchResponse {
        results: state.directory.search_by_display_name(&query.value).await,
    })
}

async fn phone_lookup(
    State(state): State<AppState>,
    Query(query): Query<LookupQuery>,
) -> Response {
    match state.directory.search_by_phone(&query.value).await {
        Some(entry) => (StatusCode::OK, Json(entry)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Debug)]
struct ApiError(AccountError);

impl From<AccountError> for ApiError {
    fn from(value: AccountError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            AccountError::DuplicateLogin
            | AccountError::DuplicatePhone
            | AccountError::DuplicateUsername => StatusCode::CONFLICT,
            AccountError::AccountNotFound | AccountError::SignInIdNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        };

        (
            status,
            Json(json!({
                "error_code": self.0.code(),
            })),
        )
            .into_response()
    }
}
