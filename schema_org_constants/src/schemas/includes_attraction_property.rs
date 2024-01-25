/// <https://schema.org/includesAttraction>
pub const INCLUDES_ATTRACTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/includesAttraction";
/// <https://schema.org/includesAttraction>
pub const INCLUDES_ATTRACTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/includesAttraction";
/// <https://schema.org/includesAttraction>
pub const INCLUDES_ATTRACTION_PROPERTY_LABEL: &str = "includesAttraction";
pub struct IncludesAttractionPropertyIri;
impl PartialEq<&str> for IncludesAttractionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDES_ATTRACTION_PROPERTY_IRI_HTTP
			|| *other == INCLUDES_ATTRACTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludesAttractionPropertyIri> for &str {
	fn eq(&self, other: &IncludesAttractionPropertyIri) -> bool {
		*self == INCLUDES_ATTRACTION_PROPERTY_IRI_HTTP
			|| *self == INCLUDES_ATTRACTION_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludesAttractionPropertyIriOrLabel;
impl PartialEq<&str> for IncludesAttractionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludesAttractionPropertyIri || *other == INCLUDES_ATTRACTION_PROPERTY_LABEL
	}
}
impl PartialEq<IncludesAttractionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludesAttractionPropertyIriOrLabel) -> bool {
		*self == IncludesAttractionPropertyIri || *self == INCLUDES_ATTRACTION_PROPERTY_LABEL
	}
}
