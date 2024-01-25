/// <https://schema.org/EUEnergyEfficiencyCategoryF>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryF";
/// <https://schema.org/EUEnergyEfficiencyCategoryF>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryF";
/// <https://schema.org/EUEnergyEfficiencyCategoryF>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_F_LABEL: &str = "EUEnergyEfficiencyCategoryF";
pub struct EuEnergyEfficiencyCategoryFIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryFIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryFIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryFIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryFIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryFIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryFIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_F_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryFIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryFIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryFIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_F_LABEL
	}
}
