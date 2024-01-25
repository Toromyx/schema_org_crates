/// <https://schema.org/saturatedFatContent>
pub const SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/saturatedFatContent";
/// <https://schema.org/saturatedFatContent>
pub const SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/saturatedFatContent";
/// <https://schema.org/saturatedFatContent>
pub const SATURATED_FAT_CONTENT_PROPERTY_LABEL: &str = "saturatedFatContent";
pub struct SaturatedFatContentPropertyIri;
impl PartialEq<&str> for SaturatedFatContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *other == SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SaturatedFatContentPropertyIri> for &str {
	fn eq(&self, other: &SaturatedFatContentPropertyIri) -> bool {
		*self == SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *self == SATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SaturatedFatContentPropertyIriOrLabel;
impl PartialEq<&str> for SaturatedFatContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SaturatedFatContentPropertyIri || *other == SATURATED_FAT_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<SaturatedFatContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SaturatedFatContentPropertyIriOrLabel) -> bool {
		*self == SaturatedFatContentPropertyIri || *self == SATURATED_FAT_CONTENT_PROPERTY_LABEL
	}
}
