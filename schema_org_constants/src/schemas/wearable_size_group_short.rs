/// <https://schema.org/WearableSizeGroupShort>
pub const WEARABLE_SIZE_GROUP_SHORT_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupShort";
/// <https://schema.org/WearableSizeGroupShort>
pub const WEARABLE_SIZE_GROUP_SHORT_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupShort";
/// <https://schema.org/WearableSizeGroupShort>
pub const WEARABLE_SIZE_GROUP_SHORT_LABEL: &str = "WearableSizeGroupShort";
pub struct WearableSizeGroupShortIri;
impl PartialEq<&str> for WearableSizeGroupShortIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_SHORT_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_SHORT_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupShortIri> for &str {
	fn eq(&self, other: &WearableSizeGroupShortIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_SHORT_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_SHORT_IRI_HTTPS
	}
}
pub struct WearableSizeGroupShortIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupShortIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupShortIri || *other == WEARABLE_SIZE_GROUP_SHORT_LABEL
	}
}
impl PartialEq<WearableSizeGroupShortIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupShortIriOrLabel) -> bool {
		*self == WearableSizeGroupShortIri || *self == WEARABLE_SIZE_GROUP_SHORT_LABEL
	}
}
