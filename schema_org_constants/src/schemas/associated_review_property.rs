/// <https://schema.org/associatedReview>
pub const ASSOCIATED_REVIEW_PROPERTY_IRI_HTTP: &str = "http://schema.org/associatedReview";
/// <https://schema.org/associatedReview>
pub const ASSOCIATED_REVIEW_PROPERTY_IRI_HTTPS: &str = "https://schema.org/associatedReview";
/// <https://schema.org/associatedReview>
pub const ASSOCIATED_REVIEW_PROPERTY_LABEL: &str = "associatedReview";
pub struct AssociatedReviewPropertyIri;
impl PartialEq<&str> for AssociatedReviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_REVIEW_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_REVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedReviewPropertyIri> for &str {
	fn eq(&self, other: &AssociatedReviewPropertyIri) -> bool {
		*self == ASSOCIATED_REVIEW_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_REVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedReviewPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedReviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedReviewPropertyIri || *other == ASSOCIATED_REVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedReviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedReviewPropertyIriOrLabel) -> bool {
		*self == AssociatedReviewPropertyIri || *self == ASSOCIATED_REVIEW_PROPERTY_LABEL
	}
}
