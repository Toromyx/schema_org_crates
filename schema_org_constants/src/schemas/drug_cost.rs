/// <https://schema.org/DrugCost>
pub const DRUG_COST_IRI_HTTP: &str = "http://schema.org/DrugCost";
/// <https://schema.org/DrugCost>
pub const DRUG_COST_IRI_HTTPS: &str = "https://schema.org/DrugCost";
/// <https://schema.org/DrugCost>
pub const DRUG_COST_LABEL: &str = "DrugCost";
pub struct DrugCostIri;
impl PartialEq<&str> for DrugCostIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRUG_COST_IRI_HTTP || *other == DRUG_COST_IRI_HTTPS
	}
}
impl PartialEq<DrugCostIri> for &str {
	fn eq(&self, other: &DrugCostIri) -> bool {
		*self == DRUG_COST_IRI_HTTP || *self == DRUG_COST_IRI_HTTPS
	}
}
pub struct DrugCostIriOrLabel;
impl PartialEq<&str> for DrugCostIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrugCostIri || *other == DRUG_COST_LABEL
	}
}
impl PartialEq<DrugCostIriOrLabel> for &str {
	fn eq(&self, other: &DrugCostIriOrLabel) -> bool {
		*self == DrugCostIri || *self == DRUG_COST_LABEL
	}
}
