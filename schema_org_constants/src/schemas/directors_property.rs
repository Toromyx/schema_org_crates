/// <https://schema.org/directors>
#[deprecated = "This schema is superseded by <https://schema.org/director>."]
pub const DIRECTORS_PROPERTY_IRI_HTTP: &str = "http://schema.org/directors";
/// <https://schema.org/directors>
#[deprecated = "This schema is superseded by <https://schema.org/director>."]
pub const DIRECTORS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/directors";
/// <https://schema.org/directors>
#[deprecated = "This schema is superseded by <https://schema.org/director>."]
pub const DIRECTORS_PROPERTY_LABEL: &str = "directors";
pub struct DirectorsPropertyIri;
impl PartialEq<&str> for DirectorsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIRECTORS_PROPERTY_IRI_HTTP || *other == DIRECTORS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DirectorsPropertyIri> for &str {
	fn eq(&self, other: &DirectorsPropertyIri) -> bool {
		*self == DIRECTORS_PROPERTY_IRI_HTTP || *self == DIRECTORS_PROPERTY_IRI_HTTPS
	}
}
pub struct DirectorsPropertyIriOrLabel;
impl PartialEq<&str> for DirectorsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DirectorsPropertyIri || *other == DIRECTORS_PROPERTY_LABEL
	}
}
impl PartialEq<DirectorsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DirectorsPropertyIriOrLabel) -> bool {
		*self == DirectorsPropertyIri || *self == DIRECTORS_PROPERTY_LABEL
	}
}
