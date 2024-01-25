/// <https://schema.org/WearableSizeGroupMisses>
pub const WEARABLE_SIZE_GROUP_MISSES_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupMisses";
/// <https://schema.org/WearableSizeGroupMisses>
pub const WEARABLE_SIZE_GROUP_MISSES_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupMisses";
/// <https://schema.org/WearableSizeGroupMisses>
pub const WEARABLE_SIZE_GROUP_MISSES_LABEL: &str = "WearableSizeGroupMisses";
pub struct WearableSizeGroupMissesIri;
impl PartialEq<&str> for WearableSizeGroupMissesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_MISSES_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_MISSES_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupMissesIri> for &str {
	fn eq(&self, other: &WearableSizeGroupMissesIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_MISSES_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_MISSES_IRI_HTTPS
	}
}
pub struct WearableSizeGroupMissesIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupMissesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupMissesIri || *other == WEARABLE_SIZE_GROUP_MISSES_LABEL
	}
}
impl PartialEq<WearableSizeGroupMissesIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupMissesIriOrLabel) -> bool {
		*self == WearableSizeGroupMissesIri || *self == WEARABLE_SIZE_GROUP_MISSES_LABEL
	}
}
