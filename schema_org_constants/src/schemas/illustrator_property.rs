/// <https://schema.org/illustrator>
pub const ILLUSTRATOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/illustrator";
/// <https://schema.org/illustrator>
pub const ILLUSTRATOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/illustrator";
/// <https://schema.org/illustrator>
pub const ILLUSTRATOR_PROPERTY_LABEL: &str = "illustrator";
pub struct IllustratorPropertyIri;
impl PartialEq<&str> for IllustratorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ILLUSTRATOR_PROPERTY_IRI_HTTP || *other == ILLUSTRATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IllustratorPropertyIri> for &str {
	fn eq(&self, other: &IllustratorPropertyIri) -> bool {
		*self == ILLUSTRATOR_PROPERTY_IRI_HTTP || *self == ILLUSTRATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct IllustratorPropertyIriOrLabel;
impl PartialEq<&str> for IllustratorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IllustratorPropertyIri || *other == ILLUSTRATOR_PROPERTY_LABEL
	}
}
impl PartialEq<IllustratorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IllustratorPropertyIriOrLabel) -> bool {
		*self == IllustratorPropertyIri || *self == ILLUSTRATOR_PROPERTY_LABEL
	}
}
