/// <https://schema.org/EnergyConsumptionDetails>
pub const ENERGY_CONSUMPTION_DETAILS_IRI_HTTP: &str = "http://schema.org/EnergyConsumptionDetails";
/// <https://schema.org/EnergyConsumptionDetails>
pub const ENERGY_CONSUMPTION_DETAILS_IRI_HTTPS: &str =
	"https://schema.org/EnergyConsumptionDetails";
/// <https://schema.org/EnergyConsumptionDetails>
pub const ENERGY_CONSUMPTION_DETAILS_LABEL: &str = "EnergyConsumptionDetails";
pub struct EnergyConsumptionDetailsIri;
impl PartialEq<&str> for EnergyConsumptionDetailsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_CONSUMPTION_DETAILS_IRI_HTTP
			|| *other == ENERGY_CONSUMPTION_DETAILS_IRI_HTTPS
	}
}
impl PartialEq<EnergyConsumptionDetailsIri> for &str {
	fn eq(&self, other: &EnergyConsumptionDetailsIri) -> bool {
		*self == ENERGY_CONSUMPTION_DETAILS_IRI_HTTP
			|| *self == ENERGY_CONSUMPTION_DETAILS_IRI_HTTPS
	}
}
pub struct EnergyConsumptionDetailsIriOrLabel;
impl PartialEq<&str> for EnergyConsumptionDetailsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyConsumptionDetailsIri || *other == ENERGY_CONSUMPTION_DETAILS_LABEL
	}
}
impl PartialEq<EnergyConsumptionDetailsIriOrLabel> for &str {
	fn eq(&self, other: &EnergyConsumptionDetailsIriOrLabel) -> bool {
		*self == EnergyConsumptionDetailsIri || *self == ENERGY_CONSUMPTION_DETAILS_LABEL
	}
}
