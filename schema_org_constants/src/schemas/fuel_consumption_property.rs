/// <https://schema.org/fuelConsumption>
pub const FUEL_CONSUMPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/fuelConsumption";
/// <https://schema.org/fuelConsumption>
pub const FUEL_CONSUMPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fuelConsumption";
/// <https://schema.org/fuelConsumption>
pub const FUEL_CONSUMPTION_PROPERTY_LABEL: &str = "fuelConsumption";
pub struct FuelConsumptionPropertyIri;
impl PartialEq<&str> for FuelConsumptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUEL_CONSUMPTION_PROPERTY_IRI_HTTP
			|| *other == FUEL_CONSUMPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FuelConsumptionPropertyIri> for &str {
	fn eq(&self, other: &FuelConsumptionPropertyIri) -> bool {
		*self == FUEL_CONSUMPTION_PROPERTY_IRI_HTTP || *self == FUEL_CONSUMPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct FuelConsumptionPropertyIriOrLabel;
impl PartialEq<&str> for FuelConsumptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FuelConsumptionPropertyIri || *other == FUEL_CONSUMPTION_PROPERTY_LABEL
	}
}
impl PartialEq<FuelConsumptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FuelConsumptionPropertyIriOrLabel) -> bool {
		*self == FuelConsumptionPropertyIri || *self == FUEL_CONSUMPTION_PROPERTY_LABEL
	}
}
