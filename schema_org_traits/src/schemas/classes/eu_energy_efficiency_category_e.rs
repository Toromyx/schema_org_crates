/// <https://schema.org/EUEnergyEfficiencyCategoryE>
pub trait FindEuEnergyEfficiencyCategoryEIds {
	type IdType;
	/// <https://schema.org/EUEnergyEfficiencyCategoryE>
	fn find_eu_energy_efficiency_category_e_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEuEnergyEfficiencyCategoryEIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_eu_energy_efficiency_category_e_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EU_ENERGY_EFFICIENCY_CATEGORY_E_IRI_HTTPS
				}
			})
		}
	}
}
