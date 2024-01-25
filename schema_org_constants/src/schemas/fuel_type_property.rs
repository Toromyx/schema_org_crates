/// <https://schema.org/fuelType>
pub const FUEL_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/fuelType";
/// <https://schema.org/fuelType>
pub const FUEL_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fuelType";
/// <https://schema.org/fuelType>
pub const FUEL_TYPE_PROPERTY_LABEL: &str = "fuelType";
pub struct FuelTypePropertyIri;
impl PartialEq<&str> for FuelTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUEL_TYPE_PROPERTY_IRI_HTTP || *other == FUEL_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FuelTypePropertyIri> for &str {
	fn eq(&self, other: &FuelTypePropertyIri) -> bool {
		*self == FUEL_TYPE_PROPERTY_IRI_HTTP || *self == FUEL_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct FuelTypePropertyIriOrLabel;
impl PartialEq<&str> for FuelTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FuelTypePropertyIri || *other == FUEL_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<FuelTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FuelTypePropertyIriOrLabel) -> bool {
		*self == FuelTypePropertyIri || *self == FUEL_TYPE_PROPERTY_LABEL
	}
}
