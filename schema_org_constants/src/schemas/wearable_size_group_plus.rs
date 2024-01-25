/// <https://schema.org/WearableSizeGroupPlus>
pub const WEARABLE_SIZE_GROUP_PLUS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupPlus";
/// <https://schema.org/WearableSizeGroupPlus>
pub const WEARABLE_SIZE_GROUP_PLUS_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupPlus";
/// <https://schema.org/WearableSizeGroupPlus>
pub const WEARABLE_SIZE_GROUP_PLUS_LABEL: &str = "WearableSizeGroupPlus";
pub struct WearableSizeGroupPlusIri;
impl PartialEq<&str> for WearableSizeGroupPlusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_PLUS_IRI_HTTP || *other == WEARABLE_SIZE_GROUP_PLUS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupPlusIri> for &str {
	fn eq(&self, other: &WearableSizeGroupPlusIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_PLUS_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_PLUS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupPlusIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupPlusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupPlusIri || *other == WEARABLE_SIZE_GROUP_PLUS_LABEL
	}
}
impl PartialEq<WearableSizeGroupPlusIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupPlusIriOrLabel) -> bool {
		*self == WearableSizeGroupPlusIri || *self == WEARABLE_SIZE_GROUP_PLUS_LABEL
	}
}
