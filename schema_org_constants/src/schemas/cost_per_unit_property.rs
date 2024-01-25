/// <https://schema.org/costPerUnit>
pub const COST_PER_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/costPerUnit";
/// <https://schema.org/costPerUnit>
pub const COST_PER_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/costPerUnit";
/// <https://schema.org/costPerUnit>
pub const COST_PER_UNIT_PROPERTY_LABEL: &str = "costPerUnit";
pub struct CostPerUnitPropertyIri;
impl PartialEq<&str> for CostPerUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COST_PER_UNIT_PROPERTY_IRI_HTTP || *other == COST_PER_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CostPerUnitPropertyIri> for &str {
	fn eq(&self, other: &CostPerUnitPropertyIri) -> bool {
		*self == COST_PER_UNIT_PROPERTY_IRI_HTTP || *self == COST_PER_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct CostPerUnitPropertyIriOrLabel;
impl PartialEq<&str> for CostPerUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CostPerUnitPropertyIri || *other == COST_PER_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<CostPerUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CostPerUnitPropertyIriOrLabel) -> bool {
		*self == CostPerUnitPropertyIri || *self == COST_PER_UNIT_PROPERTY_LABEL
	}
}
