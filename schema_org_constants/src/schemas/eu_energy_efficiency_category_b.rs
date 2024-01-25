/// <https://schema.org/EUEnergyEfficiencyCategoryB>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryB";
/// <https://schema.org/EUEnergyEfficiencyCategoryB>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryB";
/// <https://schema.org/EUEnergyEfficiencyCategoryB>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_B_LABEL: &str = "EUEnergyEfficiencyCategoryB";
pub struct EuEnergyEfficiencyCategoryBIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryBIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryBIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryBIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_B_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryBIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryBIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryBIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_B_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryBIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryBIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryBIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_B_LABEL
	}
}
