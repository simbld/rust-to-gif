use serde::{Deserialize, Serialize};

// DTO for login requests
#[derive(Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

// DTO for registration requests
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
  pub login: String,
  pub username: String,
  pub email: String,
  pub password: String,
}

// DTO for updating user password
#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
  pub email: String,
  pub new_password: String,
}

// DTO for authentication response
#[derive(Serialize)]
pub struct AuthResponse {
  pub success: bool,
  pub token: Option<String>,
  pub message: Option<String>,
}

// DTO for general error responses in authentication
#[derive(Serialize)]
pub struct AuthErrorResponse {
  pub error: String,
}
