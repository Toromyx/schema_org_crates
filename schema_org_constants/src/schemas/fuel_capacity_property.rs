/// <https://schema.org/fuelCapacity>
pub const FUEL_CAPACITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/fuelCapacity";
/// <https://schema.org/fuelCapacity>
pub const FUEL_CAPACITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fuelCapacity";
/// <https://schema.org/fuelCapacity>
pub const FUEL_CAPACITY_PROPERTY_LABEL: &str = "fuelCapacity";
pub struct FuelCapacityPropertyIri;
impl PartialEq<&str> for FuelCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUEL_CAPACITY_PROPERTY_IRI_HTTP || *other == FUEL_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FuelCapacityPropertyIri> for &str {
	fn eq(&self, other: &FuelCapacityPropertyIri) -> bool {
		*self == FUEL_CAPACITY_PROPERTY_IRI_HTTP || *self == FUEL_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct FuelCapacityPropertyIriOrLabel;
impl PartialEq<&str> for FuelCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FuelCapacityPropertyIri || *other == FUEL_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<FuelCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FuelCapacityPropertyIriOrLabel) -> bool {
		*self == FuelCapacityPropertyIri || *self == FUEL_CAPACITY_PROPERTY_LABEL
	}
}
