/// <https://schema.org/WearableSizeSystemDE>
pub const WEARABLE_SIZE_SYSTEM_DE_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemDE";
/// <https://schema.org/WearableSizeSystemDE>
pub const WEARABLE_SIZE_SYSTEM_DE_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemDE";
/// <https://schema.org/WearableSizeSystemDE>
pub const WEARABLE_SIZE_SYSTEM_DE_LABEL: &str = "WearableSizeSystemDE";
pub struct WearableSizeSystemDeIri;
impl PartialEq<&str> for WearableSizeSystemDeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_DE_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_DE_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemDeIri> for &str {
	fn eq(&self, other: &WearableSizeSystemDeIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_DE_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_DE_IRI_HTTPS
	}
}
pub struct WearableSizeSystemDeIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemDeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemDeIri || *other == WEARABLE_SIZE_SYSTEM_DE_LABEL
	}
}
impl PartialEq<WearableSizeSystemDeIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemDeIriOrLabel) -> bool {
		*self == WearableSizeSystemDeIri || *self == WEARABLE_SIZE_SYSTEM_DE_LABEL
	}
}
