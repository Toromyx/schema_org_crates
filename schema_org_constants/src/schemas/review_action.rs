/// <https://schema.org/ReviewAction>
pub const REVIEW_ACTION_IRI_HTTP: &str = "http://schema.org/ReviewAction";
/// <https://schema.org/ReviewAction>
pub const REVIEW_ACTION_IRI_HTTPS: &str = "https://schema.org/ReviewAction";
/// <https://schema.org/ReviewAction>
pub const REVIEW_ACTION_LABEL: &str = "ReviewAction";
pub struct ReviewActionIri;
impl PartialEq<&str> for ReviewActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_ACTION_IRI_HTTP || *other == REVIEW_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReviewActionIri> for &str {
	fn eq(&self, other: &ReviewActionIri) -> bool {
		*self == REVIEW_ACTION_IRI_HTTP || *self == REVIEW_ACTION_IRI_HTTPS
	}
}
pub struct ReviewActionIriOrLabel;
impl PartialEq<&str> for ReviewActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewActionIri || *other == REVIEW_ACTION_LABEL
	}
}
impl PartialEq<ReviewActionIriOrLabel> for &str {
	fn eq(&self, other: &ReviewActionIriOrLabel) -> bool {
		*self == ReviewActionIri || *self == REVIEW_ACTION_LABEL
	}
}
