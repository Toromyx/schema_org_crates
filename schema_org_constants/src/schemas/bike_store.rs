/// <https://schema.org/BikeStore>
pub const BIKE_STORE_IRI_HTTP: &str = "http://schema.org/BikeStore";
/// <https://schema.org/BikeStore>
pub const BIKE_STORE_IRI_HTTPS: &str = "https://schema.org/BikeStore";
/// <https://schema.org/BikeStore>
pub const BIKE_STORE_LABEL: &str = "BikeStore";
pub struct BikeStoreIri;
impl PartialEq<&str> for BikeStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIKE_STORE_IRI_HTTP || *other == BIKE_STORE_IRI_HTTPS
	}
}
impl PartialEq<BikeStoreIri> for &str {
	fn eq(&self, other: &BikeStoreIri) -> bool {
		*self == BIKE_STORE_IRI_HTTP || *self == BIKE_STORE_IRI_HTTPS
	}
}
pub struct BikeStoreIriOrLabel;
impl PartialEq<&str> for BikeStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BikeStoreIri || *other == BIKE_STORE_LABEL
	}
}
impl PartialEq<BikeStoreIriOrLabel> for &str {
	fn eq(&self, other: &BikeStoreIriOrLabel) -> bool {
		*self == BikeStoreIri || *self == BIKE_STORE_LABEL
	}
}
