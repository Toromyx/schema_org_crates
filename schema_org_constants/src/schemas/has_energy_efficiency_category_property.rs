/// <https://schema.org/hasEnergyEfficiencyCategory>
pub const HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasEnergyEfficiencyCategory";
/// <https://schema.org/hasEnergyEfficiencyCategory>
pub const HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasEnergyEfficiencyCategory";
/// <https://schema.org/hasEnergyEfficiencyCategory>
pub const HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_LABEL: &str = "hasEnergyEfficiencyCategory";
pub struct HasEnergyEfficiencyCategoryPropertyIri;
impl PartialEq<&str> for HasEnergyEfficiencyCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasEnergyEfficiencyCategoryPropertyIri> for &str {
	fn eq(&self, other: &HasEnergyEfficiencyCategoryPropertyIri) -> bool {
		*self == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct HasEnergyEfficiencyCategoryPropertyIriOrLabel;
impl PartialEq<&str> for HasEnergyEfficiencyCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasEnergyEfficiencyCategoryPropertyIri
			|| *other == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<HasEnergyEfficiencyCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasEnergyEfficiencyCategoryPropertyIriOrLabel) -> bool {
		*self == HasEnergyEfficiencyCategoryPropertyIri
			|| *self == HAS_ENERGY_EFFICIENCY_CATEGORY_PROPERTY_LABEL
	}
}
