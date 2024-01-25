/// <https://schema.org/EnergyEfficiencyEnumeration>
pub const ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/EnergyEfficiencyEnumeration";
/// <https://schema.org/EnergyEfficiencyEnumeration>
pub const ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/EnergyEfficiencyEnumeration";
/// <https://schema.org/EnergyEfficiencyEnumeration>
pub const ENERGY_EFFICIENCY_ENUMERATION_LABEL: &str = "EnergyEfficiencyEnumeration";
pub struct EnergyEfficiencyEnumerationIri;
impl PartialEq<&str> for EnergyEfficiencyEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *other == ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<EnergyEfficiencyEnumerationIri> for &str {
	fn eq(&self, other: &EnergyEfficiencyEnumerationIri) -> bool {
		*self == ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *self == ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
pub struct EnergyEfficiencyEnumerationIriOrLabel;
impl PartialEq<&str> for EnergyEfficiencyEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyEfficiencyEnumerationIri || *other == ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
impl PartialEq<EnergyEfficiencyEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &EnergyEfficiencyEnumerationIriOrLabel) -> bool {
		*self == EnergyEfficiencyEnumerationIri || *self == ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
