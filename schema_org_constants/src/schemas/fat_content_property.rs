/// <https://schema.org/fatContent>
pub const FAT_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/fatContent";
/// <https://schema.org/fatContent>
pub const FAT_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fatContent";
/// <https://schema.org/fatContent>
pub const FAT_CONTENT_PROPERTY_LABEL: &str = "fatContent";
pub struct FatContentPropertyIri;
impl PartialEq<&str> for FatContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAT_CONTENT_PROPERTY_IRI_HTTP || *other == FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FatContentPropertyIri> for &str {
	fn eq(&self, other: &FatContentPropertyIri) -> bool {
		*self == FAT_CONTENT_PROPERTY_IRI_HTTP || *self == FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct FatContentPropertyIriOrLabel;
impl PartialEq<&str> for FatContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FatContentPropertyIri || *other == FAT_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<FatContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FatContentPropertyIriOrLabel) -> bool {
		*self == FatContentPropertyIri || *self == FAT_CONTENT_PROPERTY_LABEL
	}
}
