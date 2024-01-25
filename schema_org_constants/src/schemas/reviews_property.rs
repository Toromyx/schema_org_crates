/// <https://schema.org/reviews>
#[deprecated = "This schema is superseded by <https://schema.org/review>."]
pub const REVIEWS_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviews";
/// <https://schema.org/reviews>
#[deprecated = "This schema is superseded by <https://schema.org/review>."]
pub const REVIEWS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviews";
/// <https://schema.org/reviews>
#[deprecated = "This schema is superseded by <https://schema.org/review>."]
pub const REVIEWS_PROPERTY_LABEL: &str = "reviews";
pub struct ReviewsPropertyIri;
impl PartialEq<&str> for ReviewsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEWS_PROPERTY_IRI_HTTP || *other == REVIEWS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewsPropertyIri> for &str {
	fn eq(&self, other: &ReviewsPropertyIri) -> bool {
		*self == REVIEWS_PROPERTY_IRI_HTTP || *self == REVIEWS_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewsPropertyIriOrLabel;
impl PartialEq<&str> for ReviewsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewsPropertyIri || *other == REVIEWS_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewsPropertyIriOrLabel) -> bool {
		*self == ReviewsPropertyIri || *self == REVIEWS_PROPERTY_LABEL
	}
}
