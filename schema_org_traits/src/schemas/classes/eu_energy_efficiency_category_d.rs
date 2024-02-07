/// <https://schema.org/EUEnergyEfficiencyCategoryD>
pub trait FindEuEnergyEfficiencyCategoryDIds {
	type IdType;
	/// <https://schema.org/EUEnergyEfficiencyCategoryD>
	fn find_eu_energy_efficiency_category_d_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEuEnergyEfficiencyCategoryDIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_eu_energy_efficiency_category_d_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_D_IRI_HTTPS
				}
			})
		}
	}
}
