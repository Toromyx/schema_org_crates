/// <https://schema.org/DrugCost>
pub trait FindDrugCostIds {
	type IdType;
	/// <https://schema.org/DrugCost>
	fn find_drug_cost_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDrugCostIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_drug_cost_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DRUG_COST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DRUG_COST_IRI_HTTPS,
			})
		}
	}
}
