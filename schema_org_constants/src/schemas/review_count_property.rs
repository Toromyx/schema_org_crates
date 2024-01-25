/// <https://schema.org/reviewCount>
pub const REVIEW_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviewCount";
/// <https://schema.org/reviewCount>
pub const REVIEW_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviewCount";
/// <https://schema.org/reviewCount>
pub const REVIEW_COUNT_PROPERTY_LABEL: &str = "reviewCount";
pub struct ReviewCountPropertyIri;
impl PartialEq<&str> for ReviewCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_COUNT_PROPERTY_IRI_HTTP || *other == REVIEW_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewCountPropertyIri> for &str {
	fn eq(&self, other: &ReviewCountPropertyIri) -> bool {
		*self == REVIEW_COUNT_PROPERTY_IRI_HTTP || *self == REVIEW_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewCountPropertyIriOrLabel;
impl PartialEq<&str> for ReviewCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewCountPropertyIri || *other == REVIEW_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewCountPropertyIriOrLabel) -> bool {
		*self == ReviewCountPropertyIri || *self == REVIEW_COUNT_PROPERTY_LABEL
	}
}
