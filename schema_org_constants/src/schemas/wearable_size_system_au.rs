/// <https://schema.org/WearableSizeSystemAU>
pub const WEARABLE_SIZE_SYSTEM_AU_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemAU";
/// <https://schema.org/WearableSizeSystemAU>
pub const WEARABLE_SIZE_SYSTEM_AU_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemAU";
/// <https://schema.org/WearableSizeSystemAU>
pub const WEARABLE_SIZE_SYSTEM_AU_LABEL: &str = "WearableSizeSystemAU";
pub struct WearableSizeSystemAuIri;
impl PartialEq<&str> for WearableSizeSystemAuIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_AU_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_AU_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemAuIri> for &str {
	fn eq(&self, other: &WearableSizeSystemAuIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_AU_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_AU_IRI_HTTPS
	}
}
pub struct WearableSizeSystemAuIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemAuIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemAuIri || *other == WEARABLE_SIZE_SYSTEM_AU_LABEL
	}
}
impl PartialEq<WearableSizeSystemAuIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemAuIriOrLabel) -> bool {
		*self == WearableSizeSystemAuIri || *self == WEARABLE_SIZE_SYSTEM_AU_LABEL
	}
}
