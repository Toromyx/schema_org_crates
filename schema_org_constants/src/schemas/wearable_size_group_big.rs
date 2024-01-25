/// <https://schema.org/WearableSizeGroupBig>
pub const WEARABLE_SIZE_GROUP_BIG_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupBig";
/// <https://schema.org/WearableSizeGroupBig>
pub const WEARABLE_SIZE_GROUP_BIG_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupBig";
/// <https://schema.org/WearableSizeGroupBig>
pub const WEARABLE_SIZE_GROUP_BIG_LABEL: &str = "WearableSizeGroupBig";
pub struct WearableSizeGroupBigIri;
impl PartialEq<&str> for WearableSizeGroupBigIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_BIG_IRI_HTTP || *other == WEARABLE_SIZE_GROUP_BIG_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupBigIri> for &str {
	fn eq(&self, other: &WearableSizeGroupBigIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_BIG_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_BIG_IRI_HTTPS
	}
}
pub struct WearableSizeGroupBigIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupBigIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupBigIri || *other == WEARABLE_SIZE_GROUP_BIG_LABEL
	}
}
impl PartialEq<WearableSizeGroupBigIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupBigIriOrLabel) -> bool {
		*self == WearableSizeGroupBigIri || *self == WEARABLE_SIZE_GROUP_BIG_LABEL
	}
}
