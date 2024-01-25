/// <https://schema.org/MediaReviewItem>
pub const MEDIA_REVIEW_ITEM_IRI_HTTP: &str = "http://schema.org/MediaReviewItem";
/// <https://schema.org/MediaReviewItem>
pub const MEDIA_REVIEW_ITEM_IRI_HTTPS: &str = "https://schema.org/MediaReviewItem";
/// <https://schema.org/MediaReviewItem>
pub const MEDIA_REVIEW_ITEM_LABEL: &str = "MediaReviewItem";
pub struct MediaReviewItemIri;
impl PartialEq<&str> for MediaReviewItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_REVIEW_ITEM_IRI_HTTP || *other == MEDIA_REVIEW_ITEM_IRI_HTTPS
	}
}
impl PartialEq<MediaReviewItemIri> for &str {
	fn eq(&self, other: &MediaReviewItemIri) -> bool {
		*self == MEDIA_REVIEW_ITEM_IRI_HTTP || *self == MEDIA_REVIEW_ITEM_IRI_HTTPS
	}
}
pub struct MediaReviewItemIriOrLabel;
impl PartialEq<&str> for MediaReviewItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaReviewItemIri || *other == MEDIA_REVIEW_ITEM_LABEL
	}
}
impl PartialEq<MediaReviewItemIriOrLabel> for &str {
	fn eq(&self, other: &MediaReviewItemIriOrLabel) -> bool {
		*self == MediaReviewItemIri || *self == MEDIA_REVIEW_ITEM_LABEL
	}
}
