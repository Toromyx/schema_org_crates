/// <https://schema.org/WearableSizeSystemMX>
pub const WEARABLE_SIZE_SYSTEM_MX_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemMX";
/// <https://schema.org/WearableSizeSystemMX>
pub const WEARABLE_SIZE_SYSTEM_MX_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemMX";
/// <https://schema.org/WearableSizeSystemMX>
pub const WEARABLE_SIZE_SYSTEM_MX_LABEL: &str = "WearableSizeSystemMX";
pub struct WearableSizeSystemMxIri;
impl PartialEq<&str> for WearableSizeSystemMxIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_MX_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_MX_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemMxIri> for &str {
	fn eq(&self, other: &WearableSizeSystemMxIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_MX_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_MX_IRI_HTTPS
	}
}
pub struct WearableSizeSystemMxIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemMxIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemMxIri || *other == WEARABLE_SIZE_SYSTEM_MX_LABEL
	}
}
impl PartialEq<WearableSizeSystemMxIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemMxIriOrLabel) -> bool {
		*self == WearableSizeSystemMxIri || *self == WEARABLE_SIZE_SYSTEM_MX_LABEL
	}
}
