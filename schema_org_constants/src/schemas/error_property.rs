/// <https://schema.org/error>
pub const ERROR_PROPERTY_IRI_HTTP: &str = "http://schema.org/error";
/// <https://schema.org/error>
pub const ERROR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/error";
/// <https://schema.org/error>
pub const ERROR_PROPERTY_LABEL: &str = "error";
pub struct ErrorPropertyIri;
impl PartialEq<&str> for ErrorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ERROR_PROPERTY_IRI_HTTP || *other == ERROR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ErrorPropertyIri> for &str {
	fn eq(&self, other: &ErrorPropertyIri) -> bool {
		*self == ERROR_PROPERTY_IRI_HTTP || *self == ERROR_PROPERTY_IRI_HTTPS
	}
}
pub struct ErrorPropertyIriOrLabel;
impl PartialEq<&str> for ErrorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ErrorPropertyIri || *other == ERROR_PROPERTY_LABEL
	}
}
impl PartialEq<ErrorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ErrorPropertyIriOrLabel) -> bool {
		*self == ErrorPropertyIri || *self == ERROR_PROPERTY_LABEL
	}
}
