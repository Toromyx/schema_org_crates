/// <https://schema.org/EndorsementRating>
pub const ENDORSEMENT_RATING_IRI_HTTP: &str = "http://schema.org/EndorsementRating";
/// <https://schema.org/EndorsementRating>
pub const ENDORSEMENT_RATING_IRI_HTTPS: &str = "https://schema.org/EndorsementRating";
/// <https://schema.org/EndorsementRating>
pub const ENDORSEMENT_RATING_LABEL: &str = "EndorsementRating";
pub struct EndorsementRatingIri;
impl PartialEq<&str> for EndorsementRatingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENDORSEMENT_RATING_IRI_HTTP || *other == ENDORSEMENT_RATING_IRI_HTTPS
	}
}
impl PartialEq<EndorsementRatingIri> for &str {
	fn eq(&self, other: &EndorsementRatingIri) -> bool {
		*self == ENDORSEMENT_RATING_IRI_HTTP || *self == ENDORSEMENT_RATING_IRI_HTTPS
	}
}
pub struct EndorsementRatingIriOrLabel;
impl PartialEq<&str> for EndorsementRatingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndorsementRatingIri || *other == ENDORSEMENT_RATING_LABEL
	}
}
impl PartialEq<EndorsementRatingIriOrLabel> for &str {
	fn eq(&self, other: &EndorsementRatingIriOrLabel) -> bool {
		*self == EndorsementRatingIri || *self == ENDORSEMENT_RATING_LABEL
	}
}
