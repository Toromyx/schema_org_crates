/// <https://schema.org/WearableSizeSystemUS>
pub const WEARABLE_SIZE_SYSTEM_US_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemUS";
/// <https://schema.org/WearableSizeSystemUS>
pub const WEARABLE_SIZE_SYSTEM_US_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemUS";
/// <https://schema.org/WearableSizeSystemUS>
pub const WEARABLE_SIZE_SYSTEM_US_LABEL: &str = "WearableSizeSystemUS";
pub struct WearableSizeSystemUsIri;
impl PartialEq<&str> for WearableSizeSystemUsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_US_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_US_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemUsIri> for &str {
	fn eq(&self, other: &WearableSizeSystemUsIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_US_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_US_IRI_HTTPS
	}
}
pub struct WearableSizeSystemUsIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemUsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemUsIri || *other == WEARABLE_SIZE_SYSTEM_US_LABEL
	}
}
impl PartialEq<WearableSizeSystemUsIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemUsIriOrLabel) -> bool {
		*self == WearableSizeSystemUsIri || *self == WEARABLE_SIZE_SYSTEM_US_LABEL
	}
}
