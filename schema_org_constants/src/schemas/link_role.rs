/// <https://schema.org/LinkRole>
pub const LINK_ROLE_IRI_HTTP: &str = "http://schema.org/LinkRole";
/// <https://schema.org/LinkRole>
pub const LINK_ROLE_IRI_HTTPS: &str = "https://schema.org/LinkRole";
/// <https://schema.org/LinkRole>
pub const LINK_ROLE_LABEL: &str = "LinkRole";
pub struct LinkRoleIri;
impl PartialEq<&str> for LinkRoleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LINK_ROLE_IRI_HTTP || *other == LINK_ROLE_IRI_HTTPS
	}
}
impl PartialEq<LinkRoleIri> for &str {
	fn eq(&self, other: &LinkRoleIri) -> bool {
		*self == LINK_ROLE_IRI_HTTP || *self == LINK_ROLE_IRI_HTTPS
	}
}
pub struct LinkRoleIriOrLabel;
impl PartialEq<&str> for LinkRoleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LinkRoleIri || *other == LINK_ROLE_LABEL
	}
}
impl PartialEq<LinkRoleIriOrLabel> for &str {
	fn eq(&self, other: &LinkRoleIriOrLabel) -> bool {
		*self == LinkRoleIri || *self == LINK_ROLE_LABEL
	}
}
