/// <https://schema.org/costCategory>
pub const COST_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/costCategory";
/// <https://schema.org/costCategory>
pub const COST_CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/costCategory";
/// <https://schema.org/costCategory>
pub const COST_CATEGORY_PROPERTY_LABEL: &str = "costCategory";
pub struct CostCategoryPropertyIri;
impl PartialEq<&str> for CostCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COST_CATEGORY_PROPERTY_IRI_HTTP || *other == COST_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CostCategoryPropertyIri> for &str {
	fn eq(&self, other: &CostCategoryPropertyIri) -> bool {
		*self == COST_CATEGORY_PROPERTY_IRI_HTTP || *self == COST_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct CostCategoryPropertyIriOrLabel;
impl PartialEq<&str> for CostCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CostCategoryPropertyIri || *other == COST_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<CostCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CostCategoryPropertyIriOrLabel) -> bool {
		*self == CostCategoryPropertyIri || *self == COST_CATEGORY_PROPERTY_LABEL
	}
}
