/// <https://schema.org/permissionType>
pub const PERMISSION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/permissionType";
/// <https://schema.org/permissionType>
pub const PERMISSION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/permissionType";
/// <https://schema.org/permissionType>
pub const PERMISSION_TYPE_PROPERTY_LABEL: &str = "permissionType";
pub struct PermissionTypePropertyIri;
impl PartialEq<&str> for PermissionTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERMISSION_TYPE_PROPERTY_IRI_HTTP || *other == PERMISSION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PermissionTypePropertyIri> for &str {
	fn eq(&self, other: &PermissionTypePropertyIri) -> bool {
		*self == PERMISSION_TYPE_PROPERTY_IRI_HTTP || *self == PERMISSION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PermissionTypePropertyIriOrLabel;
impl PartialEq<&str> for PermissionTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PermissionTypePropertyIri || *other == PERMISSION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PermissionTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PermissionTypePropertyIriOrLabel) -> bool {
		*self == PermissionTypePropertyIri || *self == PERMISSION_TYPE_PROPERTY_LABEL
	}
}
