/// <https://schema.org/contentRating>
pub const CONTENT_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentRating";
/// <https://schema.org/contentRating>
pub const CONTENT_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contentRating";
/// <https://schema.org/contentRating>
pub const CONTENT_RATING_PROPERTY_LABEL: &str = "contentRating";
pub struct ContentRatingPropertyIri;
impl PartialEq<&str> for ContentRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_RATING_PROPERTY_IRI_HTTP || *other == CONTENT_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentRatingPropertyIri> for &str {
	fn eq(&self, other: &ContentRatingPropertyIri) -> bool {
		*self == CONTENT_RATING_PROPERTY_IRI_HTTP || *self == CONTENT_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentRatingPropertyIriOrLabel;
impl PartialEq<&str> for ContentRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentRatingPropertyIri || *other == CONTENT_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<ContentRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentRatingPropertyIriOrLabel) -> bool {
		*self == ContentRatingPropertyIri || *self == CONTENT_RATING_PROPERTY_LABEL
	}
}
