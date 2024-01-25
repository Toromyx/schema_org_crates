/// <https://schema.org/isSimilarTo>
pub const IS_SIMILAR_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/isSimilarTo";
/// <https://schema.org/isSimilarTo>
pub const IS_SIMILAR_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isSimilarTo";
/// <https://schema.org/isSimilarTo>
pub const IS_SIMILAR_TO_PROPERTY_LABEL: &str = "isSimilarTo";
pub struct IsSimilarToPropertyIri;
impl PartialEq<&str> for IsSimilarToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_SIMILAR_TO_PROPERTY_IRI_HTTP || *other == IS_SIMILAR_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsSimilarToPropertyIri> for &str {
	fn eq(&self, other: &IsSimilarToPropertyIri) -> bool {
		*self == IS_SIMILAR_TO_PROPERTY_IRI_HTTP || *self == IS_SIMILAR_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct IsSimilarToPropertyIriOrLabel;
impl PartialEq<&str> for IsSimilarToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsSimilarToPropertyIri || *other == IS_SIMILAR_TO_PROPERTY_LABEL
	}
}
impl PartialEq<IsSimilarToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsSimilarToPropertyIriOrLabel) -> bool {
		*self == IsSimilarToPropertyIri || *self == IS_SIMILAR_TO_PROPERTY_LABEL
	}
}
