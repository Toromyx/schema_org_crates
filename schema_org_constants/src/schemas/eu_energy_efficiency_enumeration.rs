/// <https://schema.org/EUEnergyEfficiencyEnumeration>
pub const EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyEnumeration";
/// <https://schema.org/EUEnergyEfficiencyEnumeration>
pub const EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyEnumeration";
/// <https://schema.org/EUEnergyEfficiencyEnumeration>
pub const EU_ENERGY_EFFICIENCY_ENUMERATION_LABEL: &str = "EUEnergyEfficiencyEnumeration";
pub struct EuEnergyEfficiencyEnumerationIri;
impl PartialEq<&str> for EuEnergyEfficiencyEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyEnumerationIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyEnumerationIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_ENUMERATION_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyEnumerationIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyEnumerationIri
			|| *other == EU_ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyEnumerationIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyEnumerationIri || *self == EU_ENERGY_EFFICIENCY_ENUMERATION_LABEL
	}
}
