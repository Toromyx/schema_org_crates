/// <https://schema.org/energyEfficiencyScaleMin>
pub const ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/energyEfficiencyScaleMin";
/// <https://schema.org/energyEfficiencyScaleMin>
pub const ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/energyEfficiencyScaleMin";
/// <https://schema.org/energyEfficiencyScaleMin>
pub const ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_LABEL: &str = "energyEfficiencyScaleMin";
pub struct EnergyEfficiencyScaleMinPropertyIri;
impl PartialEq<&str> for EnergyEfficiencyScaleMinPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTP
			|| *other == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EnergyEfficiencyScaleMinPropertyIri> for &str {
	fn eq(&self, other: &EnergyEfficiencyScaleMinPropertyIri) -> bool {
		*self == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTP
			|| *self == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_IRI_HTTPS
	}
}
pub struct EnergyEfficiencyScaleMinPropertyIriOrLabel;
impl PartialEq<&str> for EnergyEfficiencyScaleMinPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyEfficiencyScaleMinPropertyIri
			|| *other == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_LABEL
	}
}
impl PartialEq<EnergyEfficiencyScaleMinPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EnergyEfficiencyScaleMinPropertyIriOrLabel) -> bool {
		*self == EnergyEfficiencyScaleMinPropertyIri
			|| *self == ENERGY_EFFICIENCY_SCALE_MIN_PROPERTY_LABEL
	}
}
