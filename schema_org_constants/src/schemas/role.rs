/// <https://schema.org/Role>
pub const ROLE_IRI_HTTP: &str = "http://schema.org/Role";
/// <https://schema.org/Role>
pub const ROLE_IRI_HTTPS: &str = "https://schema.org/Role";
/// <https://schema.org/Role>
pub const ROLE_LABEL: &str = "Role";
pub struct RoleIri;
impl PartialEq<&str> for RoleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ROLE_IRI_HTTP || *other == ROLE_IRI_HTTPS
	}
}
impl PartialEq<RoleIri> for &str {
	fn eq(&self, other: &RoleIri) -> bool {
		*self == ROLE_IRI_HTTP || *self == ROLE_IRI_HTTPS
	}
}
pub struct RoleIriOrLabel;
impl PartialEq<&str> for RoleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RoleIri || *other == ROLE_LABEL
	}
}
impl PartialEq<RoleIriOrLabel> for &str {
	fn eq(&self, other: &RoleIriOrLabel) -> bool {
		*self == RoleIri || *self == ROLE_LABEL
	}
}
