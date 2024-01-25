/// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryA2Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryA2Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_LABEL: &str = "EUEnergyEfficiencyCategoryA2Plus";
pub struct EuEnergyEfficiencyCategoryA2PlusIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA2PlusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA2PlusIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA2PlusIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryA2PlusIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA2PlusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryA2PlusIri
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA2PlusIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA2PlusIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryA2PlusIri
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_2_PLUS_LABEL
	}
}
