/// <https://schema.org/WearableSizeGroupMaternity>
pub const WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTP: &str =
	"http://schema.org/WearableSizeGroupMaternity";
/// <https://schema.org/WearableSizeGroupMaternity>
pub const WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupMaternity";
/// <https://schema.org/WearableSizeGroupMaternity>
pub const WEARABLE_SIZE_GROUP_MATERNITY_LABEL: &str = "WearableSizeGroupMaternity";
pub struct WearableSizeGroupMaternityIri;
impl PartialEq<&str> for WearableSizeGroupMaternityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupMaternityIri> for &str {
	fn eq(&self, other: &WearableSizeGroupMaternityIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTPS
	}
}
pub struct WearableSizeGroupMaternityIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupMaternityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupMaternityIri || *other == WEARABLE_SIZE_GROUP_MATERNITY_LABEL
	}
}
impl PartialEq<WearableSizeGroupMaternityIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupMaternityIriOrLabel) -> bool {
		*self == WearableSizeGroupMaternityIri || *self == WEARABLE_SIZE_GROUP_MATERNITY_LABEL
	}
}
