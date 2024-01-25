/// <https://schema.org/associatedClaimReview>
pub const ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/associatedClaimReview";
/// <https://schema.org/associatedClaimReview>
pub const ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/associatedClaimReview";
/// <https://schema.org/associatedClaimReview>
pub const ASSOCIATED_CLAIM_REVIEW_PROPERTY_LABEL: &str = "associatedClaimReview";
pub struct AssociatedClaimReviewPropertyIri;
impl PartialEq<&str> for AssociatedClaimReviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedClaimReviewPropertyIri> for &str {
	fn eq(&self, other: &AssociatedClaimReviewPropertyIri) -> bool {
		*self == ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_CLAIM_REVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedClaimReviewPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedClaimReviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedClaimReviewPropertyIri
			|| *other == ASSOCIATED_CLAIM_REVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedClaimReviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedClaimReviewPropertyIriOrLabel) -> bool {
		*self == AssociatedClaimReviewPropertyIri || *self == ASSOCIATED_CLAIM_REVIEW_PROPERTY_LABEL
	}
}
