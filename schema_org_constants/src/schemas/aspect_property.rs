/// <https://schema.org/aspect>
#[deprecated = "This schema is superseded by <https://schema.org/mainContentOfPage>."]
pub const ASPECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/aspect";
/// <https://schema.org/aspect>
#[deprecated = "This schema is superseded by <https://schema.org/mainContentOfPage>."]
pub const ASPECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/aspect";
/// <https://schema.org/aspect>
#[deprecated = "This schema is superseded by <https://schema.org/mainContentOfPage>."]
pub const ASPECT_PROPERTY_LABEL: &str = "aspect";
pub struct AspectPropertyIri;
impl PartialEq<&str> for AspectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASPECT_PROPERTY_IRI_HTTP || *other == ASPECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AspectPropertyIri> for &str {
	fn eq(&self, other: &AspectPropertyIri) -> bool {
		*self == ASPECT_PROPERTY_IRI_HTTP || *self == ASPECT_PROPERTY_IRI_HTTPS
	}
}
pub struct AspectPropertyIriOrLabel;
impl PartialEq<&str> for AspectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AspectPropertyIri || *other == ASPECT_PROPERTY_LABEL
	}
}
impl PartialEq<AspectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AspectPropertyIriOrLabel) -> bool {
		*self == AspectPropertyIri || *self == ASPECT_PROPERTY_LABEL
	}
}
