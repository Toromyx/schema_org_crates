/// <https://schema.org/associatedMediaReview>
pub const ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/associatedMediaReview";
/// <https://schema.org/associatedMediaReview>
pub const ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/associatedMediaReview";
/// <https://schema.org/associatedMediaReview>
pub const ASSOCIATED_MEDIA_REVIEW_PROPERTY_LABEL: &str = "associatedMediaReview";
pub struct AssociatedMediaReviewPropertyIri;
impl PartialEq<&str> for AssociatedMediaReviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedMediaReviewPropertyIri> for &str {
	fn eq(&self, other: &AssociatedMediaReviewPropertyIri) -> bool {
		*self == ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_MEDIA_REVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedMediaReviewPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedMediaReviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedMediaReviewPropertyIri
			|| *other == ASSOCIATED_MEDIA_REVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedMediaReviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedMediaReviewPropertyIriOrLabel) -> bool {
		*self == AssociatedMediaReviewPropertyIri || *self == ASSOCIATED_MEDIA_REVIEW_PROPERTY_LABEL
	}
}
