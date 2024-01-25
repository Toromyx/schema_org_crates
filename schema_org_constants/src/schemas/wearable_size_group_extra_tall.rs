/// <https://schema.org/WearableSizeGroupExtraTall>
pub const WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTP: &str =
	"http://schema.org/WearableSizeGroupExtraTall";
/// <https://schema.org/WearableSizeGroupExtraTall>
pub const WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupExtraTall";
/// <https://schema.org/WearableSizeGroupExtraTall>
pub const WEARABLE_SIZE_GROUP_EXTRA_TALL_LABEL: &str = "WearableSizeGroupExtraTall";
pub struct WearableSizeGroupExtraTallIri;
impl PartialEq<&str> for WearableSizeGroupExtraTallIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupExtraTallIri> for &str {
	fn eq(&self, other: &WearableSizeGroupExtraTallIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTPS
	}
}
pub struct WearableSizeGroupExtraTallIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupExtraTallIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupExtraTallIri || *other == WEARABLE_SIZE_GROUP_EXTRA_TALL_LABEL
	}
}
impl PartialEq<WearableSizeGroupExtraTallIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupExtraTallIriOrLabel) -> bool {
		*self == WearableSizeGroupExtraTallIri || *self == WEARABLE_SIZE_GROUP_EXTRA_TALL_LABEL
	}
}
