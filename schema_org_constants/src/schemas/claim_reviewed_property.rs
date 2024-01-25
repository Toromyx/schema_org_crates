/// <https://schema.org/claimReviewed>
pub const CLAIM_REVIEWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/claimReviewed";
/// <https://schema.org/claimReviewed>
pub const CLAIM_REVIEWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/claimReviewed";
/// <https://schema.org/claimReviewed>
pub const CLAIM_REVIEWED_PROPERTY_LABEL: &str = "claimReviewed";
pub struct ClaimReviewedPropertyIri;
impl PartialEq<&str> for ClaimReviewedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLAIM_REVIEWED_PROPERTY_IRI_HTTP || *other == CLAIM_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClaimReviewedPropertyIri> for &str {
	fn eq(&self, other: &ClaimReviewedPropertyIri) -> bool {
		*self == CLAIM_REVIEWED_PROPERTY_IRI_HTTP || *self == CLAIM_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
pub struct ClaimReviewedPropertyIriOrLabel;
impl PartialEq<&str> for ClaimReviewedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClaimReviewedPropertyIri || *other == CLAIM_REVIEWED_PROPERTY_LABEL
	}
}
impl PartialEq<ClaimReviewedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClaimReviewedPropertyIriOrLabel) -> bool {
		*self == ClaimReviewedPropertyIri || *self == CLAIM_REVIEWED_PROPERTY_LABEL
	}
}
