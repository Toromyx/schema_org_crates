/// <https://schema.org/mediaAuthenticityCategory>
pub const MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/mediaAuthenticityCategory";
/// <https://schema.org/mediaAuthenticityCategory>
pub const MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/mediaAuthenticityCategory";
/// <https://schema.org/mediaAuthenticityCategory>
pub const MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_LABEL: &str = "mediaAuthenticityCategory";
pub struct MediaAuthenticityCategoryPropertyIri;
impl PartialEq<&str> for MediaAuthenticityCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MediaAuthenticityCategoryPropertyIri> for &str {
	fn eq(&self, other: &MediaAuthenticityCategoryPropertyIri) -> bool {
		*self == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct MediaAuthenticityCategoryPropertyIriOrLabel;
impl PartialEq<&str> for MediaAuthenticityCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaAuthenticityCategoryPropertyIri
			|| *other == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<MediaAuthenticityCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MediaAuthenticityCategoryPropertyIriOrLabel) -> bool {
		*self == MediaAuthenticityCategoryPropertyIri
			|| *self == MEDIA_AUTHENTICITY_CATEGORY_PROPERTY_LABEL
	}
}
