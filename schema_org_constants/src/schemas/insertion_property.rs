/// <https://schema.org/insertion>
pub const INSERTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/insertion";
/// <https://schema.org/insertion>
pub const INSERTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/insertion";
/// <https://schema.org/insertion>
pub const INSERTION_PROPERTY_LABEL: &str = "insertion";
pub struct InsertionPropertyIri;
impl PartialEq<&str> for InsertionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSERTION_PROPERTY_IRI_HTTP || *other == INSERTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InsertionPropertyIri> for &str {
	fn eq(&self, other: &InsertionPropertyIri) -> bool {
		*self == INSERTION_PROPERTY_IRI_HTTP || *self == INSERTION_PROPERTY_IRI_HTTPS
	}
}
pub struct InsertionPropertyIriOrLabel;
impl PartialEq<&str> for InsertionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InsertionPropertyIri || *other == INSERTION_PROPERTY_LABEL
	}
}
impl PartialEq<InsertionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InsertionPropertyIriOrLabel) -> bool {
		*self == InsertionPropertyIri || *self == INSERTION_PROPERTY_LABEL
	}
}
