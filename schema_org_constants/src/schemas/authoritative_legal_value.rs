/// <https://schema.org/AuthoritativeLegalValue>
pub const AUTHORITATIVE_LEGAL_VALUE_IRI_HTTP: &str = "http://schema.org/AuthoritativeLegalValue";
/// <https://schema.org/AuthoritativeLegalValue>
pub const AUTHORITATIVE_LEGAL_VALUE_IRI_HTTPS: &str = "https://schema.org/AuthoritativeLegalValue";
/// <https://schema.org/AuthoritativeLegalValue>
pub const AUTHORITATIVE_LEGAL_VALUE_LABEL: &str = "AuthoritativeLegalValue";
pub struct AuthoritativeLegalValueIri;
impl PartialEq<&str> for AuthoritativeLegalValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTHORITATIVE_LEGAL_VALUE_IRI_HTTP
			|| *other == AUTHORITATIVE_LEGAL_VALUE_IRI_HTTPS
	}
}
impl PartialEq<AuthoritativeLegalValueIri> for &str {
	fn eq(&self, other: &AuthoritativeLegalValueIri) -> bool {
		*self == AUTHORITATIVE_LEGAL_VALUE_IRI_HTTP || *self == AUTHORITATIVE_LEGAL_VALUE_IRI_HTTPS
	}
}
pub struct AuthoritativeLegalValueIriOrLabel;
impl PartialEq<&str> for AuthoritativeLegalValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AuthoritativeLegalValueIri || *other == AUTHORITATIVE_LEGAL_VALUE_LABEL
	}
}
impl PartialEq<AuthoritativeLegalValueIriOrLabel> for &str {
	fn eq(&self, other: &AuthoritativeLegalValueIriOrLabel) -> bool {
		*self == AuthoritativeLegalValueIri || *self == AUTHORITATIVE_LEGAL_VALUE_LABEL
	}
}
