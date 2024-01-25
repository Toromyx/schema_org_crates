/// <https://schema.org/grantee>
pub const GRANTEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/grantee";
/// <https://schema.org/grantee>
pub const GRANTEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/grantee";
/// <https://schema.org/grantee>
pub const GRANTEE_PROPERTY_LABEL: &str = "grantee";
pub struct GranteePropertyIri;
impl PartialEq<&str> for GranteePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GRANTEE_PROPERTY_IRI_HTTP || *other == GRANTEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GranteePropertyIri> for &str {
	fn eq(&self, other: &GranteePropertyIri) -> bool {
		*self == GRANTEE_PROPERTY_IRI_HTTP || *self == GRANTEE_PROPERTY_IRI_HTTPS
	}
}
pub struct GranteePropertyIriOrLabel;
impl PartialEq<&str> for GranteePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GranteePropertyIri || *other == GRANTEE_PROPERTY_LABEL
	}
}
impl PartialEq<GranteePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GranteePropertyIriOrLabel) -> bool {
		*self == GranteePropertyIri || *self == GRANTEE_PROPERTY_LABEL
	}
}
