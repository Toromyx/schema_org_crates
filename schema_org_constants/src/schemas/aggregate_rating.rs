/// <https://schema.org/AggregateRating>
pub const AGGREGATE_RATING_IRI_HTTP: &str = "http://schema.org/AggregateRating";
/// <https://schema.org/AggregateRating>
pub const AGGREGATE_RATING_IRI_HTTPS: &str = "https://schema.org/AggregateRating";
/// <https://schema.org/AggregateRating>
pub const AGGREGATE_RATING_LABEL: &str = "AggregateRating";
pub struct AggregateRatingIri;
impl PartialEq<&str> for AggregateRatingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGGREGATE_RATING_IRI_HTTP || *other == AGGREGATE_RATING_IRI_HTTPS
	}
}
impl PartialEq<AggregateRatingIri> for &str {
	fn eq(&self, other: &AggregateRatingIri) -> bool {
		*self == AGGREGATE_RATING_IRI_HTTP || *self == AGGREGATE_RATING_IRI_HTTPS
	}
}
pub struct AggregateRatingIriOrLabel;
impl PartialEq<&str> for AggregateRatingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AggregateRatingIri || *other == AGGREGATE_RATING_LABEL
	}
}
impl PartialEq<AggregateRatingIriOrLabel> for &str {
	fn eq(&self, other: &AggregateRatingIriOrLabel) -> bool {
		*self == AggregateRatingIri || *self == AGGREGATE_RATING_LABEL
	}
}
