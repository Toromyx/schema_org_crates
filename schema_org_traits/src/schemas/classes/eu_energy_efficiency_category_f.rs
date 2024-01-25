/// <https://schema.org/EUEnergyEfficiencyCategoryF>
pub trait FindEuEnergyEfficiencyCategoryFIds {
	type IdType;
	fn find_eu_energy_efficiency_category_f_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEuEnergyEfficiencyCategoryFIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_eu_energy_efficiency_category_f_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_F_IRI_HTTPS
				}
			})
		}
	}
}
