//! Permission handling for user roles
//!
//! This module manages role-based access control (RBAC).

use crate::error::AuthError;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Map of roles to their permissions
static ROLE_PERMISSIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Admin role has all permissions
    map.insert("admin", vec![
        "user:read", "user:write", "user:delete",
        "agent:read", "agent:write", "agent:delete",
        "system:read", "system:write", "system:admin"
    ]);
    
    // User role has limited permissions
    map.insert("user", vec![
        "user:read", 
        "agent:read", "agent:write",
        "system:read"
    ]);
    
    // ReadOnly role has only read permissions
    map.insert("readonly", vec![
        "user:read",
        "agent:read",
        "system:read"
    ]);
    
    map
});

/// Check if a role has a specific permission
pub fn check_permission(role: &str, permission: &str) -> Result<bool, AuthError> {
    let permissions = ROLE_PERMISSIONS.get(role)
        .ok_or(AuthError::InvalidRole)?;
        
    Ok(permissions.contains(&permission))
}

/// Get all permissions for a role
pub fn get_role_permissions(role: &str) -> Result<Vec<&'static str>, AuthError> {
    let permissions = ROLE_PERMISSIONS.get(role)
        .ok_or(AuthError::InvalidRole)?;
        
    Ok(permissions.clone())
}

/// Check if a user has admin privileges
pub fn is_admin(role: &str) -> bool {
    role == "admin"
}
