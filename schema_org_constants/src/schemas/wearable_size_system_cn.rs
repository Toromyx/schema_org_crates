/// <https://schema.org/WearableSizeSystemCN>
pub const WEARABLE_SIZE_SYSTEM_CN_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemCN";
/// <https://schema.org/WearableSizeSystemCN>
pub const WEARABLE_SIZE_SYSTEM_CN_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemCN";
/// <https://schema.org/WearableSizeSystemCN>
pub const WEARABLE_SIZE_SYSTEM_CN_LABEL: &str = "WearableSizeSystemCN";
pub struct WearableSizeSystemCnIri;
impl PartialEq<&str> for WearableSizeSystemCnIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_CN_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_CN_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemCnIri> for &str {
	fn eq(&self, other: &WearableSizeSystemCnIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_CN_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_CN_IRI_HTTPS
	}
}
pub struct WearableSizeSystemCnIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemCnIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemCnIri || *other == WEARABLE_SIZE_SYSTEM_CN_LABEL
	}
}
impl PartialEq<WearableSizeSystemCnIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemCnIriOrLabel) -> bool {
		*self == WearableSizeSystemCnIri || *self == WEARABLE_SIZE_SYSTEM_CN_LABEL
	}
}
