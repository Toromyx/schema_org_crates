/// <https://schema.org/WearableSizeSystemJP>
pub const WEARABLE_SIZE_SYSTEM_JP_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemJP";
/// <https://schema.org/WearableSizeSystemJP>
pub const WEARABLE_SIZE_SYSTEM_JP_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemJP";
/// <https://schema.org/WearableSizeSystemJP>
pub const WEARABLE_SIZE_SYSTEM_JP_LABEL: &str = "WearableSizeSystemJP";
pub struct WearableSizeSystemJpIri;
impl PartialEq<&str> for WearableSizeSystemJpIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_JP_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_JP_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemJpIri> for &str {
	fn eq(&self, other: &WearableSizeSystemJpIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_JP_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_JP_IRI_HTTPS
	}
}
pub struct WearableSizeSystemJpIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemJpIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemJpIri || *other == WEARABLE_SIZE_SYSTEM_JP_LABEL
	}
}
impl PartialEq<WearableSizeSystemJpIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemJpIriOrLabel) -> bool {
		*self == WearableSizeSystemJpIri || *self == WEARABLE_SIZE_SYSTEM_JP_LABEL
	}
}
