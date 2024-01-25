/// <https://schema.org/authenticator>
pub const AUTHENTICATOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/authenticator";
/// <https://schema.org/authenticator>
pub const AUTHENTICATOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/authenticator";
/// <https://schema.org/authenticator>
pub const AUTHENTICATOR_PROPERTY_LABEL: &str = "authenticator";
pub struct AuthenticatorPropertyIri;
impl PartialEq<&str> for AuthenticatorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTHENTICATOR_PROPERTY_IRI_HTTP || *other == AUTHENTICATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AuthenticatorPropertyIri> for &str {
	fn eq(&self, other: &AuthenticatorPropertyIri) -> bool {
		*self == AUTHENTICATOR_PROPERTY_IRI_HTTP || *self == AUTHENTICATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct AuthenticatorPropertyIriOrLabel;
impl PartialEq<&str> for AuthenticatorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AuthenticatorPropertyIri || *other == AUTHENTICATOR_PROPERTY_LABEL
	}
}
impl PartialEq<AuthenticatorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AuthenticatorPropertyIriOrLabel) -> bool {
		*self == AuthenticatorPropertyIri || *self == AUTHENTICATOR_PROPERTY_LABEL
	}
}
