/// <https://schema.org/WearableSizeGroupBoys>
pub const WEARABLE_SIZE_GROUP_BOYS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupBoys";
/// <https://schema.org/WearableSizeGroupBoys>
pub const WEARABLE_SIZE_GROUP_BOYS_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupBoys";
/// <https://schema.org/WearableSizeGroupBoys>
pub const WEARABLE_SIZE_GROUP_BOYS_LABEL: &str = "WearableSizeGroupBoys";
pub struct WearableSizeGroupBoysIri;
impl PartialEq<&str> for WearableSizeGroupBoysIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_BOYS_IRI_HTTP || *other == WEARABLE_SIZE_GROUP_BOYS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupBoysIri> for &str {
	fn eq(&self, other: &WearableSizeGroupBoysIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_BOYS_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_BOYS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupBoysIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupBoysIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupBoysIri || *other == WEARABLE_SIZE_GROUP_BOYS_LABEL
	}
}
impl PartialEq<WearableSizeGroupBoysIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupBoysIriOrLabel) -> bool {
		*self == WearableSizeGroupBoysIri || *self == WEARABLE_SIZE_GROUP_BOYS_LABEL
	}
}
