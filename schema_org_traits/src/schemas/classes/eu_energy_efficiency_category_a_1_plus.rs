/// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
pub trait FindEuEnergyEfficiencyCategoryA1PlusIds {
	type IdType;
	fn find_eu_energy_efficiency_category_a_1_plus_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEuEnergyEfficiencyCategoryA1PlusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_eu_energy_efficiency_category_a_1_plus_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_A_1_PLUS_IRI_HTTPS
				}
			})
		}
	}
}
