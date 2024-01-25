/// <https://schema.org/mediaItemAppearance>
pub const MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/mediaItemAppearance";
/// <https://schema.org/mediaItemAppearance>
pub const MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mediaItemAppearance";
/// <https://schema.org/mediaItemAppearance>
pub const MEDIA_ITEM_APPEARANCE_PROPERTY_LABEL: &str = "mediaItemAppearance";
pub struct MediaItemAppearancePropertyIri;
impl PartialEq<&str> for MediaItemAppearancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTP
			|| *other == MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MediaItemAppearancePropertyIri> for &str {
	fn eq(&self, other: &MediaItemAppearancePropertyIri) -> bool {
		*self == MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTP
			|| *self == MEDIA_ITEM_APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct MediaItemAppearancePropertyIriOrLabel;
impl PartialEq<&str> for MediaItemAppearancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaItemAppearancePropertyIri || *other == MEDIA_ITEM_APPEARANCE_PROPERTY_LABEL
	}
}
impl PartialEq<MediaItemAppearancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MediaItemAppearancePropertyIriOrLabel) -> bool {
		*self == MediaItemAppearancePropertyIri || *self == MEDIA_ITEM_APPEARANCE_PROPERTY_LABEL
	}
}
