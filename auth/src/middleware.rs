use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use headers::{authorization::Bearer, Authorization};
use std::sync::Arc;
use crate::service::{AuthService, Claims, AuthError};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub roles: Vec<String>,
}

#[derive(Debug)]
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn authenticate(&self, request: Request, next: Next) -> Result<Response, StatusCode> {
        let (mut parts, body) = request.into_parts();
        
        // Extract token from headers
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Validate token
        let claims = self.auth_service
            .validate_token(token)
            .map_err(|e| match e {
                AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
                _ => StatusCode::FORBIDDEN,
            })?;

        // Inject user into request extensions
        parts.extensions.insert(AuthenticatedUser {
            id: claims.sub,
            roles: claims.roles,
        });

        // Continue processing the request
        let request = Request::from_parts(parts, body);
        Ok(next.run(request).await)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Debug, Clone)]
pub struct RoleGuard {
    required_roles: Vec<String>,
}

impl RoleGuard {
    pub fn new(required_roles: Vec<String>) -> Self {
        Self { required_roles }
    }

    pub fn has_required_role(&self, user: &AuthenticatedUser) -> bool {
        self.required_roles
            .iter()
            .any(|required_role| user.roles.contains(required_role))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RoleGuard
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<RoleGuard>()
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
