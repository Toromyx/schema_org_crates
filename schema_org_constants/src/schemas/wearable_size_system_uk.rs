/// <https://schema.org/WearableSizeSystemUK>
pub const WEARABLE_SIZE_SYSTEM_UK_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemUK";
/// <https://schema.org/WearableSizeSystemUK>
pub const WEARABLE_SIZE_SYSTEM_UK_IRI_HTTPS: &str = "https://schema.org/WearableSizeSystemUK";
/// <https://schema.org/WearableSizeSystemUK>
pub const WEARABLE_SIZE_SYSTEM_UK_LABEL: &str = "WearableSizeSystemUK";
pub struct WearableSizeSystemUkIri;
impl PartialEq<&str> for WearableSizeSystemUkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_UK_IRI_HTTP || *other == WEARABLE_SIZE_SYSTEM_UK_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemUkIri> for &str {
	fn eq(&self, other: &WearableSizeSystemUkIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_UK_IRI_HTTP || *self == WEARABLE_SIZE_SYSTEM_UK_IRI_HTTPS
	}
}
pub struct WearableSizeSystemUkIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemUkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemUkIri || *other == WEARABLE_SIZE_SYSTEM_UK_LABEL
	}
}
impl PartialEq<WearableSizeSystemUkIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemUkIriOrLabel) -> bool {
		*self == WearableSizeSystemUkIri || *self == WEARABLE_SIZE_SYSTEM_UK_LABEL
	}
}
