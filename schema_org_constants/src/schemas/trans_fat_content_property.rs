/// <https://schema.org/transFatContent>
pub const TRANS_FAT_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/transFatContent";
/// <https://schema.org/transFatContent>
pub const TRANS_FAT_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/transFatContent";
/// <https://schema.org/transFatContent>
pub const TRANS_FAT_CONTENT_PROPERTY_LABEL: &str = "transFatContent";
pub struct TransFatContentPropertyIri;
impl PartialEq<&str> for TransFatContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANS_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *other == TRANS_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TransFatContentPropertyIri> for &str {
	fn eq(&self, other: &TransFatContentPropertyIri) -> bool {
		*self == TRANS_FAT_CONTENT_PROPERTY_IRI_HTTP
			|| *self == TRANS_FAT_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct TransFatContentPropertyIriOrLabel;
impl PartialEq<&str> for TransFatContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransFatContentPropertyIri || *other == TRANS_FAT_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<TransFatContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TransFatContentPropertyIriOrLabel) -> bool {
		*self == TransFatContentPropertyIri || *self == TRANS_FAT_CONTENT_PROPERTY_LABEL
	}
}
