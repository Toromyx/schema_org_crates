/// <https://schema.org/isbn>
pub const ISBN_PROPERTY_IRI_HTTP: &str = "http://schema.org/isbn";
/// <https://schema.org/isbn>
pub const ISBN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isbn";
/// <https://schema.org/isbn>
pub const ISBN_PROPERTY_LABEL: &str = "isbn";
pub struct IsbnPropertyIri;
impl PartialEq<&str> for IsbnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISBN_PROPERTY_IRI_HTTP || *other == ISBN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsbnPropertyIri> for &str {
	fn eq(&self, other: &IsbnPropertyIri) -> bool {
		*self == ISBN_PROPERTY_IRI_HTTP || *self == ISBN_PROPERTY_IRI_HTTPS
	}
}
pub struct IsbnPropertyIriOrLabel;
impl PartialEq<&str> for IsbnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsbnPropertyIri || *other == ISBN_PROPERTY_LABEL
	}
}
impl PartialEq<IsbnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsbnPropertyIriOrLabel) -> bool {
		*self == IsbnPropertyIri || *self == ISBN_PROPERTY_LABEL
	}
}
