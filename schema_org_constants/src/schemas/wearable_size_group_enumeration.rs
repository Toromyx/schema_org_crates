/// <https://schema.org/WearableSizeGroupEnumeration>
pub const WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/WearableSizeGroupEnumeration";
/// <https://schema.org/WearableSizeGroupEnumeration>
pub const WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupEnumeration";
/// <https://schema.org/WearableSizeGroupEnumeration>
pub const WEARABLE_SIZE_GROUP_ENUMERATION_LABEL: &str = "WearableSizeGroupEnumeration";
pub struct WearableSizeGroupEnumerationIri;
impl PartialEq<&str> for WearableSizeGroupEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupEnumerationIri> for &str {
	fn eq(&self, other: &WearableSizeGroupEnumerationIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_ENUMERATION_IRI_HTTPS
	}
}
pub struct WearableSizeGroupEnumerationIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupEnumerationIri || *other == WEARABLE_SIZE_GROUP_ENUMERATION_LABEL
	}
}
impl PartialEq<WearableSizeGroupEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupEnumerationIriOrLabel) -> bool {
		*self == WearableSizeGroupEnumerationIri || *self == WEARABLE_SIZE_GROUP_ENUMERATION_LABEL
	}
}
