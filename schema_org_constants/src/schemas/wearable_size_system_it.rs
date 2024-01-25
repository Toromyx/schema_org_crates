/// <https://schema.org/WearableSizeSystemIT>
pub const WEARABLE_SIZE_SYSTEM_IT_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemIT";
/// <https://schema.org/WearableSizeSystemIT>
pub const WEARABLE_SIZE_SYSTEM_IT_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemIT";
/// <https://schema.org/WearableSizeSystemIT>
pub const WEARABLE_SIZE_SYSTEM_IT_LABEL: &str = "WearableSizeSystemIT";
pub struct WearableSizeSystemItIri;
impl PartialEq<&str> for WearableSizeSystemItIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_IT_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_IT_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemItIri> for &str {
	fn eq(&self, other: &WearableSizeSystemItIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_IT_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_IT_IRI_HTTPS
	}
}
pub struct WearableSizeSystemItIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemItIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemItIri || *other == WEARABLE_SIZE_SYSTEM_IT_LABEL
	}
}
impl PartialEq<WearableSizeSystemItIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemItIriOrLabel) -> bool {
		*self == WearableSizeSystemItIri || *self == WEARABLE_SIZE_SYSTEM_IT_LABEL
	}
}
