/// <https://schema.org/hasEnergyConsumptionDetails>
pub const HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasEnergyConsumptionDetails";
/// <https://schema.org/hasEnergyConsumptionDetails>
pub const HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasEnergyConsumptionDetails";
/// <https://schema.org/hasEnergyConsumptionDetails>
pub const HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_LABEL: &str = "hasEnergyConsumptionDetails";
pub struct HasEnergyConsumptionDetailsPropertyIri;
impl PartialEq<&str> for HasEnergyConsumptionDetailsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTP
			|| *other == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasEnergyConsumptionDetailsPropertyIri> for &str {
	fn eq(&self, other: &HasEnergyConsumptionDetailsPropertyIri) -> bool {
		*self == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTP
			|| *self == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_IRI_HTTPS
	}
}
pub struct HasEnergyConsumptionDetailsPropertyIriOrLabel;
impl PartialEq<&str> for HasEnergyConsumptionDetailsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasEnergyConsumptionDetailsPropertyIri
			|| *other == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_LABEL
	}
}
impl PartialEq<HasEnergyConsumptionDetailsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasEnergyConsumptionDetailsPropertyIriOrLabel) -> bool {
		*self == HasEnergyConsumptionDetailsPropertyIri
			|| *self == HAS_ENERGY_CONSUMPTION_DETAILS_PROPERTY_LABEL
	}
}
