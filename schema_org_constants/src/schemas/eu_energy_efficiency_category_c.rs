/// <https://schema.org/EUEnergyEfficiencyCategoryC>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTP: &str =
	"http://schema.org/EUEnergyEfficiencyCategoryC";
/// <https://schema.org/EUEnergyEfficiencyCategoryC>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTPS: &str =
	"https://schema.org/EUEnergyEfficiencyCategoryC";
/// <https://schema.org/EUEnergyEfficiencyCategoryC>
pub const EU_ENERGY_EFFICIENCY_CATEGORY_C_LABEL: &str = "EUEnergyEfficiencyCategoryC";
pub struct EuEnergyEfficiencyCategoryCIri;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryCIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTP
			|| *other == EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTPS
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryCIri> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryCIri) -> bool {
		*self == EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTP
			|| *self == EU_ENERGY_EFFICIENCY_CATEGORY_C_IRI_HTTPS
	}
}
pub struct EuEnergyEfficiencyCategoryCIriOrLabel;
impl PartialEq<&str> for EuEnergyEfficiencyCategoryCIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EuEnergyEfficiencyCategoryCIri || *other == EU_ENERGY_EFFICIENCY_CATEGORY_C_LABEL
	}
}
impl PartialEq<EuEnergyEfficiencyCategoryCIriOrLabel> for &str {
	fn eq(&self, other: &EuEnergyEfficiencyCategoryCIriOrLabel) -> bool {
		*self == EuEnergyEfficiencyCategoryCIri || *self == EU_ENERGY_EFFICIENCY_CATEGORY_C_LABEL
	}
}
