/// <https://schema.org/DrugCostCategory>
pub const DRUG_COST_CATEGORY_IRI_HTTP: &str = "http://schema.org/DrugCostCategory";
/// <https://schema.org/DrugCostCategory>
pub const DRUG_COST_CATEGORY_IRI_HTTPS: &str = "https://schema.org/DrugCostCategory";
/// <https://schema.org/DrugCostCategory>
pub const DRUG_COST_CATEGORY_LABEL: &str = "DrugCostCategory";
pub struct DrugCostCategoryIri;
impl PartialEq<&str> for DrugCostCategoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_COST_CATEGORY_IRI_HTTP || *other == DRUG_COST_CATEGORY_IRI_HTTPS
	}
}
impl PartialEq<DrugCostCategoryIri> for &str {
	fn eq(&self, other: &DrugCostCategoryIri) -> bool {
		*self == DRUG_COST_CATEGORY_IRI_HTTP || *self == DRUG_COST_CATEGORY_IRI_HTTPS
	}
}
pub struct DrugCostCategoryIriOrLabel;
impl PartialEq<&str> for DrugCostCategoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugCostCategoryIri || *other == DRUG_COST_CATEGORY_LABEL
	}
}
impl PartialEq<DrugCostCategoryIriOrLabel> for &str {
	fn eq(&self, other: &DrugCostCategoryIriOrLabel) -> bool {
		*self == DrugCostCategoryIri || *self == DRUG_COST_CATEGORY_LABEL
	}
}
