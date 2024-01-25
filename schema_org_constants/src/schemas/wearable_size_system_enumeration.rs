/// <https://schema.org/WearableSizeSystemEnumeration>
pub const WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/WearableSizeSystemEnumeration";
/// <https://schema.org/WearableSizeSystemEnumeration>
pub const WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeSystemEnumeration";
/// <https://schema.org/WearableSizeSystemEnumeration>
pub const WEARABLE_SIZE_SYSTEM_ENUMERATION_LABEL: &str = "WearableSizeSystemEnumeration";
pub struct WearableSizeSystemEnumerationIri;
impl PartialEq<&str> for WearableSizeSystemEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTP
			|| *other == WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemEnumerationIri> for &str {
	fn eq(&self, other: &WearableSizeSystemEnumerationIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTP
			|| *self == WEARABLE_SIZE_SYSTEM_ENUMERATION_IRI_HTTPS
	}
}
pub struct WearableSizeSystemEnumerationIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemEnumerationIri
			|| *other == WEARABLE_SIZE_SYSTEM_ENUMERATION_LABEL
	}
}
impl PartialEq<WearableSizeSystemEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemEnumerationIriOrLabel) -> bool {
		*self == WearableSizeSystemEnumerationIri || *self == WEARABLE_SIZE_SYSTEM_ENUMERATION_LABEL
	}
}
