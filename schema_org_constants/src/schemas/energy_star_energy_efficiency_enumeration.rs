/// <https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
pub const ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/EnergyStarEnergyEfficiencyEnumeration";
/// <https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
pub const ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/EnergyStarEnergyEfficiencyEnumeration";
/// <https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
pub const ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_LABEL: &str =
	"EnergyStarEnergyEfficiencyEnumeration";
pub struct EnergyStarEnergyEfficiencyEnumerationIri;
impl PartialEq<&str> for EnergyStarEnergyEfficiencyEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *other == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<EnergyStarEnergyEfficiencyEnumerationIri> for &str {
	fn eq(&self, other: &EnergyStarEnergyEfficiencyEnumerationIri) -> bool {
		*self == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *self == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
pub struct EnergyStarEnergyEfficiencyEnumerationIriOrLabel;
impl PartialEq<&str> for EnergyStarEnergyEfficiencyEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyStarEnergyEfficiencyEnumerationIri
			|| *other == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
impl PartialEq<EnergyStarEnergyEfficiencyEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &EnergyStarEnergyEfficiencyEnumerationIriOrLabel) -> bool {
		*self == EnergyStarEnergyEfficiencyEnumerationIri
			|| *self == ENERGY_STAR_ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
