/// <https://schema.org/energyEfficiencyScaleMax>
pub const ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/energyEfficiencyScaleMax";
/// <https://schema.org/energyEfficiencyScaleMax>
pub const ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/energyEfficiencyScaleMax";
/// <https://schema.org/energyEfficiencyScaleMax>
pub const ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_LABEL: &str = "energyEfficiencyScaleMax";
pub struct EnergyEfficiencyScaleMaxPropertyIri;
impl PartialEq<&str> for EnergyEfficiencyScaleMaxPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTP
			|| *other == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EnergyEfficiencyScaleMaxPropertyIri> for &str {
	fn eq(&self, other: &EnergyEfficiencyScaleMaxPropertyIri) -> bool {
		*self == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTP
			|| *self == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_IRI_HTTPS
	}
}
pub struct EnergyEfficiencyScaleMaxPropertyIriOrLabel;
impl PartialEq<&str> for EnergyEfficiencyScaleMaxPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyEfficiencyScaleMaxPropertyIri
			|| *other == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_LABEL
	}
}
impl PartialEq<EnergyEfficiencyScaleMaxPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EnergyEfficiencyScaleMaxPropertyIriOrLabel) -> bool {
		*self == EnergyEfficiencyScaleMaxPropertyIri
			|| *self == ENERGY_EFFICIENCY_SCALE_MAX_PROPERTY_LABEL
	}
}
