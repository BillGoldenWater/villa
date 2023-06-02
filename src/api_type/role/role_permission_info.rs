use serde::Deserialize;

use crate::api_type::role::role_permission::RolePermission;

/// role permission information
#[derive(Debug, Deserialize)]
pub struct RolePermissionInfo {
  /// the permission
  #[serde(rename = "key")]
  pub perm: RolePermission,
  /// name
  pub name: String,
  /// description
  #[serde(rename = "describe")]
  pub description: String,
}
