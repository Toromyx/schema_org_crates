/// <https://schema.org/permissions>
pub const PERMISSIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/permissions";
/// <https://schema.org/permissions>
pub const PERMISSIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/permissions";
/// <https://schema.org/permissions>
pub const PERMISSIONS_PROPERTY_LABEL: &str = "permissions";
pub struct PermissionsPropertyIri;
impl PartialEq<&str> for PermissionsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERMISSIONS_PROPERTY_IRI_HTTP || *other == PERMISSIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PermissionsPropertyIri> for &str {
	fn eq(&self, other: &PermissionsPropertyIri) -> bool {
		*self == PERMISSIONS_PROPERTY_IRI_HTTP || *self == PERMISSIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct PermissionsPropertyIriOrLabel;
impl PartialEq<&str> for PermissionsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PermissionsPropertyIri || *other == PERMISSIONS_PROPERTY_LABEL
	}
}
impl PartialEq<PermissionsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PermissionsPropertyIriOrLabel) -> bool {
		*self == PermissionsPropertyIri || *self == PERMISSIONS_PROPERTY_LABEL
	}
}
