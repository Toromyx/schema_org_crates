/// <https://schema.org/EUEnergyEfficiencyCategoryE>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryE";
/// <https://schema.org/EUEnergyEfficiencyCategoryE>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryE";
/// <https://schema.org/EUEnergyEfficiencyCategoryE>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_E_LABEL: &str = "EUEnergyEfficiencyCategoryE";
pub struct EuEnergyEfficiencyCategoryEIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryEIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryEIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryEIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryEIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryEIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryEIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_E_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryEIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryEIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryEIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_E_LABEL
	}
}
