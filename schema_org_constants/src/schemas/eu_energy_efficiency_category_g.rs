/// <https://schema.org/EUEnergyEfficiencyCategoryG>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryG";
/// <https://schema.org/EUEnergyEfficiencyCategoryG>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryG";
/// <https://schema.org/EUEnergyEfficiencyCategoryG>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_G_LABEL: &str = "EUEnergyEfficiencyCategoryG";
pub struct EuEnergyEfficiencyCategoryGIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryGIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryGIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryGIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_G_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryGIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryGIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryGIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_G_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryGIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryGIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryGIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_G_LABEL
	}
}
