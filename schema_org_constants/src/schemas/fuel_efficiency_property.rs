/// <https://schema.org/fuelEfficiency>
pub const FUEL_EFFICIENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/fuelEfficiency";
/// <https://schema.org/fuelEfficiency>
pub const FUEL_EFFICIENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fuelEfficiency";
/// <https://schema.org/fuelEfficiency>
pub const FUEL_EFFICIENCY_PROPERTY_LABEL: &str = "fuelEfficiency";
pub struct FuelEfficiencyPropertyIri;
impl PartialEq<&str> for FuelEfficiencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUEL_EFFICIENCY_PROPERTY_IRI_HTTP || *other == FUEL_EFFICIENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FuelEfficiencyPropertyIri> for &str {
	fn eq(&self, other: &FuelEfficiencyPropertyIri) -> bool {
		*self == FUEL_EFFICIENCY_PROPERTY_IRI_HTTP || *self == FUEL_EFFICIENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct FuelEfficiencyPropertyIriOrLabel;
impl PartialEq<&str> for FuelEfficiencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FuelEfficiencyPropertyIri || *other == FUEL_EFFICIENCY_PROPERTY_LABEL
	}
}
impl PartialEq<FuelEfficiencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FuelEfficiencyPropertyIriOrLabel) -> bool {
		*self == FuelEfficiencyPropertyIri || *self == FUEL_EFFICIENCY_PROPERTY_LABEL
	}
}
