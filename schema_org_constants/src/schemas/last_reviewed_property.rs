/// <https://schema.org/lastReviewed>
pub const LAST_REVIEWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/lastReviewed";
/// <https://schema.org/lastReviewed>
pub const LAST_REVIEWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lastReviewed";
/// <https://schema.org/lastReviewed>
pub const LAST_REVIEWED_PROPERTY_LABEL: &str = "lastReviewed";
pub struct LastReviewedPropertyIri;
impl PartialEq<&str> for LastReviewedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LAST_REVIEWED_PROPERTY_IRI_HTTP || *other == LAST_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LastReviewedPropertyIri> for &str {
	fn eq(&self, other: &LastReviewedPropertyIri) -> bool {
		*self == LAST_REVIEWED_PROPERTY_IRI_HTTP || *self == LAST_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
pub struct LastReviewedPropertyIriOrLabel;
impl PartialEq<&str> for LastReviewedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LastReviewedPropertyIri || *other == LAST_REVIEWED_PROPERTY_LABEL
	}
}
impl PartialEq<LastReviewedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LastReviewedPropertyIriOrLabel) -> bool {
		*self == LastReviewedPropertyIri || *self == LAST_REVIEWED_PROPERTY_LABEL
	}
}
