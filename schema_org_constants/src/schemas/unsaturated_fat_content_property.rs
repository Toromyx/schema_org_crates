/// <https://schema.org/unsaturatedFatContent>
pub const UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/unsaturatedFatContent";
/// <https://schema.org/unsaturatedFatContent>
pub const UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/unsaturatedFatContent";
/// <https://schema.org/unsaturatedFatContent>
pub const UNSATURATED_FAT_CONTENT_PROPERTY_LABEL: &str = "unsaturatedFatContent";
pub struct UnsaturatedFatContentPropertyIri;
impl PartialEq<&str> for UnsaturatedFatContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *other == UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UnsaturatedFatContentPropertyIri> for &str {
	fn eq(&self, other: &UnsaturatedFatContentPropertyIri) -> bool {
		*self == UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *self == UNSATURATED_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct UnsaturatedFatContentPropertyIriOrLabel;
impl PartialEq<&str> for UnsaturatedFatContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnsaturatedFatContentPropertyIri
			|| *other == UNSATURATED_FAT_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<UnsaturatedFatContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UnsaturatedFatContentPropertyIriOrLabel) -> bool {
		*self == UnsaturatedFatContentPropertyIri || *self == UNSATURATED_FAT_CONTENT_PROPERTY_LABEL
	}
}
