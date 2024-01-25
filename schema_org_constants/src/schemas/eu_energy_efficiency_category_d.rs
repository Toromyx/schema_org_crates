/// <https://schema.org/EUEnergyEfficiencyCategoryD>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryD";
/// <https://schema.org/EUEnergyEfficiencyCategoryD>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryD";
/// <https://schema.org/EUEnergyEfficiencyCategoryD>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_D_LABEL: &str = "EUEnergyEfficiencyCategoryD";
pub struct EuEnergyEfficiencyCategoryDIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryDIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryDIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryDIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryDIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryDIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryDIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_D_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryDIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryDIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryDIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_D_LABEL
	}
}
