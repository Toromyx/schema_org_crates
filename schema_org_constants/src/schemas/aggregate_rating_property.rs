/// <https://schema.org/aggregateRating>
pub const AGGREGATE_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/aggregateRating";
/// <https://schema.org/aggregateRating>
pub const AGGREGATE_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/aggregateRating";
/// <https://schema.org/aggregateRating>
pub const AGGREGATE_RATING_PROPERTY_LABEL: &str = "aggregateRating";
pub struct AggregateRatingPropertyIri;
impl PartialEq<&str> for AggregateRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGGREGATE_RATING_PROPERTY_IRI_HTTP
			|| *other == AGGREGATE_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AggregateRatingPropertyIri> for &str {
	fn eq(&self, other: &AggregateRatingPropertyIri) -> bool {
		*self == AGGREGATE_RATING_PROPERTY_IRI_HTTP || *self == AGGREGATE_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct AggregateRatingPropertyIriOrLabel;
impl PartialEq<&str> for AggregateRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AggregateRatingPropertyIri || *other == AGGREGATE_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<AggregateRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AggregateRatingPropertyIriOrLabel) -> bool {
		*self == AggregateRatingPropertyIri || *self == AGGREGATE_RATING_PROPERTY_LABEL
	}
}
