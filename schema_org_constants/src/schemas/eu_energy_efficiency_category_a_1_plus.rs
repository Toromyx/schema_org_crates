/// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryA1Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryA1Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_LABEL: &str = "EUEnergyEfficiencyCategoryA1Plus";
pub struct EuEnergyEfficiencyCategoryA1PlusIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA1PlusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA1PlusIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA1PlusIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryA1PlusIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA1PlusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryA1PlusIri
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA1PlusIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA1PlusIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryA1PlusIri
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_LABEL
	}
}
