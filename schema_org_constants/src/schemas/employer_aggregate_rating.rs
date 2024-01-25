/// <https://schema.org/EmployerAggregateRating>
pub const EMPLOYER_AGGREGATE_RATING_IRI_HTTP: &str = "http://schema.org/EmployerAggregateRating";
/// <https://schema.org/EmployerAggregateRating>
pub const EMPLOYER_AGGREGATE_RATING_IRI_HTTPS: &str = "https://schema.org/EmployerAggregateRating";
/// <https://schema.org/EmployerAggregateRating>
pub const EMPLOYER_AGGREGATE_RATING_LABEL: &str = "EmployerAggregateRating";
pub struct EmployerAggregateRatingIri;
impl PartialEq<&str> for EmployerAggregateRatingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYER_AGGREGATE_RATING_IRI_HTTP
			|| *other == EMPLOYER_AGGREGATE_RATING_IRI_HTTPS
	}
}
impl PartialEq<EmployerAggregateRatingIri> for &str {
	fn eq(&self, other: &EmployerAggregateRatingIri) -> bool {
		*self == EMPLOYER_AGGREGATE_RATING_IRI_HTTP || *self == EMPLOYER_AGGREGATE_RATING_IRI_HTTPS
	}
}
pub struct EmployerAggregateRatingIriOrLabel;
impl PartialEq<&str> for EmployerAggregateRatingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployerAggregateRatingIri || *other == EMPLOYER_AGGREGATE_RATING_LABEL
	}
}
impl PartialEq<EmployerAggregateRatingIriOrLabel> for &str {
	fn eq(&self, other: &EmployerAggregateRatingIriOrLabel) -> bool {
		*self == EmployerAggregateRatingIri || *self == EMPLOYER_AGGREGATE_RATING_LABEL
	}
}
