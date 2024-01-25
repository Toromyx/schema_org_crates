/// <https://schema.org/WearableSizeGroupExtraShort>
pub const WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTP: &str =
	"http://schema.org/WearableSizeGroupExtraShort";
/// <https://schema.org/WearableSizeGroupExtraShort>
pub const WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupExtraShort";
/// <https://schema.org/WearableSizeGroupExtraShort>
pub const WEARABLE_SIZE_GROUP_EXTRA_SHORT_LABEL: &str = "WearableSizeGroupExtraShort";
pub struct WearableSizeGroupExtraShortIri;
impl PartialEq<&str> for WearableSizeGroupExtraShortIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupExtraShortIri> for &str {
	fn eq(&self, other: &WearableSizeGroupExtraShortIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_EXTRA_SHORT_IRI_HTTPS
	}
}
pub struct WearableSizeGroupExtraShortIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupExtraShortIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupExtraShortIri || *other == WEARABLE_SIZE_GROUP_EXTRA_SHORT_LABEL
	}
}
impl PartialEq<WearableSizeGroupExtraShortIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupExtraShortIriOrLabel) -> bool {
		*self == WearableSizeGroupExtraShortIri || *self == WEARABLE_SIZE_GROUP_EXTRA_SHORT_LABEL
	}
}
