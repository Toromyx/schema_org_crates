/// <https://schema.org/MediaReview>
pub const MEDIA_REVIEW_IRI_HTTP: &str = "http://schema.org/MediaReview";
/// <https://schema.org/MediaReview>
pub const MEDIA_REVIEW_IRI_HTTPS: &str = "https://schema.org/MediaReview";
/// <https://schema.org/MediaReview>
pub const MEDIA_REVIEW_LABEL: &str = "MediaReview";
pub struct MediaReviewIri;
impl PartialEq<&str> for MediaReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_REVIEW_IRI_HTTP || *other == MEDIA_REVIEW_IRI_HTTPS
	}
}
impl PartialEq<MediaReviewIri> for &str {
	fn eq(&self, other: &MediaReviewIri) -> bool {
		*self == MEDIA_REVIEW_IRI_HTTP || *self == MEDIA_REVIEW_IRI_HTTPS
	}
}
pub struct MediaReviewIriOrLabel;
impl PartialEq<&str> for MediaReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaReviewIri || *other == MEDIA_REVIEW_LABEL
	}
}
impl PartialEq<MediaReviewIriOrLabel> for &str {
	fn eq(&self, other: &MediaReviewIriOrLabel) -> bool {
		*self == MediaReviewIri || *self == MEDIA_REVIEW_LABEL
	}
}
