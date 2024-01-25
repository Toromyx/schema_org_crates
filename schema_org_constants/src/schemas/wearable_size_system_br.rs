/// <https://schema.org/WearableSizeSystemBR>
pub const WEARABLE_SIZE_SYSTEM_BR_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemBR";
/// <https://schema.org/WearableSizeSystemBR>
pub const WEARABLE_SIZE_SYSTEM_BR_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemBR";
/// <https://schema.org/WearableSizeSystemBR>
pub const WEARABLE_SIZE_SYSTEM_BR_LABEL: &str = "WearableSizeSystemBR";
pub struct WearableSizeSystemBrIri;
impl PartialEq<&str> for WearableSizeSystemBrIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_BR_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_BR_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemBrIri> for &str {
	fn eq(&self, other: &WearableSizeSystemBrIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_BR_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_BR_IRI_HTTPS
	}
}
pub struct WearableSizeSystemBrIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemBrIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemBrIri || *other == WEARABLE_SIZE_SYSTEM_BR_LABEL
	}
}
impl PartialEq<WearableSizeSystemBrIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemBrIriOrLabel) -> bool {
		*self == WearableSizeSystemBrIri || *self == WEARABLE_SIZE_SYSTEM_BR_LABEL
	}
}
