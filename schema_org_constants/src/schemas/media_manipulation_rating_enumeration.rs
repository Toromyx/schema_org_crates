/// <https://schema.org/MediaManipulationRatingEnumeration>
pub const MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/MediaManipulationRatingEnumeration";
/// <https://schema.org/MediaManipulationRatingEnumeration>
pub const MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/MediaManipulationRatingEnumeration";
/// <https://schema.org/MediaManipulationRatingEnumeration>
pub const MEDIA_MANIPULATION_RATING_ENUMERATION_LABEL: &str = "MediaManipulationRatingEnumeration";
pub struct MediaManipulationRatingEnumerationIri;
impl PartialEq<&str> for MediaManipulationRatingEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTP
			|| *other == MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<MediaManipulationRatingEnumerationIri> for &str {
	fn eq(&self, other: &MediaManipulationRatingEnumerationIri) -> bool {
		*self == MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTP
			|| *self == MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTPS
	}
}
pub struct MediaManipulationRatingEnumerationIriOrLabel;
impl PartialEq<&str> for MediaManipulationRatingEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaManipulationRatingEnumerationIri
			|| *other == MEDIA_MANIPULATION_RATING_ENUMERATION_LABEL
	}
}
impl PartialEq<MediaManipulationRatingEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &MediaManipulationRatingEnumerationIriOrLabel) -> bool {
		*self == MediaManipulationRatingEnumerationIri
			|| *self == MEDIA_MANIPULATION_RATING_ENUMERATION_LABEL
	}
}
