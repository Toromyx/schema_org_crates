/// <https://schema.org/EUEnergyEfficiencyCategoryA>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryA";
/// <https://schema.org/EUEnergyEfficiencyCategoryA>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryA";
/// <https://schema.org/EUEnergyEfficiencyCategoryA>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_LABEL: &str = "EUEnergyEfficiencyCategoryA";
pub struct EuEnergyEfficiencyCategoryAIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryAIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryAIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryAIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryAIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryAIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryAIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryAIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryAIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryAIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_LABEL
	}
}
