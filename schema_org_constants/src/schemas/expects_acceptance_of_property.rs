/// <https://schema.org/expectsAcceptanceOf>
pub const EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/expectsAcceptanceOf";
/// <https://schema.org/expectsAcceptanceOf>
pub const EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/expectsAcceptanceOf";
/// <https://schema.org/expectsAcceptanceOf>
pub const EXPECTS_ACCEPTANCE_OF_PROPERTY_LABEL: &str = "expectsAcceptanceOf";
pub struct ExpectsAcceptanceOfPropertyIri;
impl PartialEq<&str> for ExpectsAcceptanceOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTP
			|| *other == EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpectsAcceptanceOfPropertyIri> for &str {
	fn eq(&self, other: &ExpectsAcceptanceOfPropertyIri) -> bool {
		*self == EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTP
			|| *self == EXPECTS_ACCEPTANCE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpectsAcceptanceOfPropertyIriOrLabel;
impl PartialEq<&str> for ExpectsAcceptanceOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpectsAcceptanceOfPropertyIri || *other == EXPECTS_ACCEPTANCE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<ExpectsAcceptanceOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpectsAcceptanceOfPropertyIriOrLabel) -> bool {
		*self == ExpectsAcceptanceOfPropertyIri || *self == EXPECTS_ACCEPTANCE_OF_PROPERTY_LABEL
	}
}
