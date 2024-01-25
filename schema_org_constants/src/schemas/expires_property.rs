/// <https://schema.org/expires>
pub const EXPIRES_PROPERTY_IRI_HTTP: &str = "http://schema.org/expires";
/// <https://schema.org/expires>
pub const EXPIRES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/expires";
/// <https://schema.org/expires>
pub const EXPIRES_PROPERTY_LABEL: &str = "expires";
pub struct ExpiresPropertyIri;
impl PartialEq<&str> for ExpiresPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPIRES_PROPERTY_IRI_HTTP || *other == EXPIRES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpiresPropertyIri> for &str {
	fn eq(&self, other: &ExpiresPropertyIri) -> bool {
		*self == EXPIRES_PROPERTY_IRI_HTTP || *self == EXPIRES_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpiresPropertyIriOrLabel;
impl PartialEq<&str> for ExpiresPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpiresPropertyIri || *other == EXPIRES_PROPERTY_LABEL
	}
}
impl PartialEq<ExpiresPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpiresPropertyIriOrLabel) -> bool {
		*self == ExpiresPropertyIri || *self == EXPIRES_PROPERTY_LABEL
	}
}
