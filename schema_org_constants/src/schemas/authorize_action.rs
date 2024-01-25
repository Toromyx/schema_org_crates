/// <https://schema.org/AuthorizeAction>
pub const AUTHORIZE_ACTION_IRI_HTTP: &str = "http://schema.org/AuthorizeAction";
/// <https://schema.org/AuthorizeAction>
pub const AUTHORIZE_ACTION_IRI_HTTPS: &str = "https://schema.org/AuthorizeAction";
/// <https://schema.org/AuthorizeAction>
pub const AUTHORIZE_ACTION_LABEL: &str = "AuthorizeAction";
pub struct AuthorizeActionIri;
impl PartialEq<&str> for AuthorizeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTHORIZE_ACTION_IRI_HTTP || *other == AUTHORIZE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AuthorizeActionIri> for &str {
	fn eq(&self, other: &AuthorizeActionIri) -> bool {
		*self == AUTHORIZE_ACTION_IRI_HTTP || *self == AUTHORIZE_ACTION_IRI_HTTPS
	}
}
pub struct AuthorizeActionIriOrLabel;
impl PartialEq<&str> for AuthorizeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AuthorizeActionIri || *other == AUTHORIZE_ACTION_LABEL
	}
}
impl PartialEq<AuthorizeActionIriOrLabel> for &str {
	fn eq(&self, other: &AuthorizeActionIriOrLabel) -> bool {
		*self == AuthorizeActionIri || *self == AUTHORIZE_ACTION_LABEL
	}
}
