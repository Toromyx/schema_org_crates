/// <https://schema.org/WearableSizeGroupTall>
pub const WEARABLE_SIZE_GROUP_TALL_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupTall";
/// <https://schema.org/WearableSizeGroupTall>
pub const WEARABLE_SIZE_GROUP_TALL_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupTall";
/// <https://schema.org/WearableSizeGroupTall>
pub const WEARABLE_SIZE_GROUP_TALL_LABEL: &str = "WearableSizeGroupTall";
pub struct WearableSizeGroupTallIri;
impl PartialEq<&str> for WearableSizeGroupTallIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_TALL_IRI_HTTP || *other == WEARABLE_SIZE_GROUP_TALL_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupTallIri> for &str {
	fn eq(&self, other: &WearableSizeGroupTallIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_TALL_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_TALL_IRI_HTTPS
	}
}
pub struct WearableSizeGroupTallIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupTallIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupTallIri || *other == WEARABLE_SIZE_GROUP_TALL_LABEL
	}
}
impl PartialEq<WearableSizeGroupTallIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupTallIriOrLabel) -> bool {
		*self == WearableSizeGroupTallIri || *self == WEARABLE_SIZE_GROUP_TALL_LABEL
	}
}
