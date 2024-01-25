/// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryA3Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryA3Plus";
/// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_LABEL: &str = "EUEnergyEfficiencyCategoryA3Plus";
pub struct EuEnergyEfficiencyCategoryA3PlusIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA3PlusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA3PlusIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA3PlusIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryA3PlusIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryA3PlusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryA3PlusIri
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryA3PlusIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryA3PlusIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryA3PlusIri
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_A_3_PLUS_LABEL
	}
}
