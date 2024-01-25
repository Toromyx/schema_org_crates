/// <https://schema.org/WearableSizeGroupHusky>
pub const WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupHusky";
/// <https://schema.org/WearableSizeGroupHusky>
pub const WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupHusky";
/// <https://schema.org/WearableSizeGroupHusky>
pub const WEARABLE_SIZE_GROUP_HUSKY_LABEL: &str = "WearableSizeGroupHusky";
pub struct WearableSizeGroupHuskyIri;
impl PartialEq<&str> for WearableSizeGroupHuskyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupHuskyIri> for &str {
	fn eq(&self, other: &WearableSizeGroupHuskyIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTPS
	}
}
pub struct WearableSizeGroupHuskyIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupHuskyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupHuskyIri || *other == WEARABLE_SIZE_GROUP_HUSKY_LABEL
	}
}
impl PartialEq<WearableSizeGroupHuskyIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupHuskyIriOrLabel) -> bool {
		*self == WearableSizeGroupHuskyIri || *self == WEARABLE_SIZE_GROUP_HUSKY_LABEL
	}
}
