/// <https://schema.org/roleName>
pub const ROLE_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/roleName";
/// <https://schema.org/roleName>
pub const ROLE_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/roleName";
/// <https://schema.org/roleName>
pub const ROLE_NAME_PROPERTY_LABEL: &str = "roleName";
pub struct RoleNamePropertyIri;
impl PartialEq<&str> for RoleNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ROLE_NAME_PROPERTY_IRI_HTTP || *other == ROLE_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RoleNamePropertyIri> for &str {
	fn eq(&self, other: &RoleNamePropertyIri) -> bool {
		*self == ROLE_NAME_PROPERTY_IRI_HTTP || *self == ROLE_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct RoleNamePropertyIriOrLabel;
impl PartialEq<&str> for RoleNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RoleNamePropertyIri || *other == ROLE_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<RoleNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RoleNamePropertyIriOrLabel) -> bool {
		*self == RoleNamePropertyIri || *self == ROLE_NAME_PROPERTY_LABEL
	}
}
