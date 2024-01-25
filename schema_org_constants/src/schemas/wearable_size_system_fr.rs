/// <https://schema.org/WearableSizeSystemFR>
pub const WEARABLE_SIZE_SYSTEM_FR_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemFR";
/// <https://schema.org/WearableSizeSystemFR>
pub const WEARABLE_SIZE_SYSTEM_FR_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemFR";
/// <https://schema.org/WearableSizeSystemFR>
pub const WEARABLE_SIZE_SYSTEM_FR_LABEL: &str = "WearableSizeSystemFR";
pub struct WearableSizeSystemFrIri;
impl PartialEq<&str> for WearableSizeSystemFrIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_FR_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_FR_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemFrIri> for &str {
	fn eq(&self, other: &WearableSizeSystemFrIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_FR_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_FR_IRI_HTTPS
	}
}
pub struct WearableSizeSystemFrIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemFrIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemFrIri || *other == WEARABLE_SIZE_SYSTEM_FR_LABEL
	}
}
impl PartialEq<WearableSizeSystemFrIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemFrIriOrLabel) -> bool {
		*self == WearableSizeSystemFrIri || *self == WEARABLE_SIZE_SYSTEM_FR_LABEL
	}
}
