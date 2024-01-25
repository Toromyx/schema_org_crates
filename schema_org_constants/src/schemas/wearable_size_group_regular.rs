/// <https://schema.org/WearableSizeGroupRegular>
pub const WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupRegular";
/// <https://schema.org/WearableSizeGroupRegular>
pub const WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupRegular";
/// <https://schema.org/WearableSizeGroupRegular>
pub const WEARABLE_SIZE_GROUP_REGULAR_LABEL: &str = "WearableSizeGroupRegular";
pub struct WearableSizeGroupRegularIri;
impl PartialEq<&str> for WearableSizeGroupRegularIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupRegularIri> for &str {
	fn eq(&self, other: &WearableSizeGroupRegularIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTPS
	}
}
pub struct WearableSizeGroupRegularIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupRegularIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupRegularIri || *other == WEARABLE_SIZE_GROUP_REGULAR_LABEL
	}
}
impl PartialEq<WearableSizeGroupRegularIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupRegularIriOrLabel) -> bool {
		*self == WearableSizeGroupRegularIri || *self == WEARABLE_SIZE_GROUP_REGULAR_LABEL
	}
}
