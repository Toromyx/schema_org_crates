/// <https://schema.org/GardenStore>
pub const GARDEN_STORE_IRI_HTTP: &str = "http://schema.org/GardenStore";
/// <https://schema.org/GardenStore>
pub const GARDEN_STORE_IRI_HTTPS: &str = "https://schema.org/GardenStore";
/// <https://schema.org/GardenStore>
pub const GARDEN_STORE_LABEL: &str = "GardenStore";
pub struct GardenStoreIri;
impl PartialEq<&str> for GardenStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GARDEN_STORE_IRI_HTTP || *other == GARDEN_STORE_IRI_HTTPS
	}
}
impl PartialEq<GardenStoreIri> for &str {
	fn eq(&self, other: &GardenStoreIri) -> bool {
		*self == GARDEN_STORE_IRI_HTTP || *self == GARDEN_STORE_IRI_HTTPS
	}
}
pub struct GardenStoreIriOrLabel;
impl PartialEq<&str> for GardenStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GardenStoreIri || *other == GARDEN_STORE_LABEL
	}
}
impl PartialEq<GardenStoreIriOrLabel> for &str {
	fn eq(&self, other: &GardenStoreIriOrLabel) -> bool {
		*self == GardenStoreIri || *self == GARDEN_STORE_LABEL
	}
}
