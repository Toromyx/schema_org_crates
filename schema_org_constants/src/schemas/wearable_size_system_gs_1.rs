/// <https://schema.org/WearableSizeSystemGS1>
pub const WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemGS1";
/// <https://schema.org/WearableSizeSystemGS1>
pub const WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemGS1";
/// <https://schema.org/WearableSizeSystemGS1>
pub const WEARABLE_SIZE_SYSTEM_GS_1_LABEL: &str = "WearableSizeSystemGS1";
pub struct WearableSizeSystemGs1Iri;
impl PartialEq<&str> for WearableSizeSystemGs1Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTP
			|| *other == WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemGs1Iri> for &str {
	fn eq(&self, other: &WearableSizeSystemGs1Iri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTPS
	}
}
pub struct WearableSizeSystemGs1IriOrLabel;
impl PartialEq<&str> for WearableSizeSystemGs1IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemGs1Iri || *other == WEARABLE_SIZE_SYSTEM_GS_1_LABEL
	}
}
impl PartialEq<WearableSizeSystemGs1IriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemGs1IriOrLabel) -> bool {
		*self == WearableSizeSystemGs1Iri || *self == WEARABLE_SIZE_SYSTEM_GS_1_LABEL
	}
}
