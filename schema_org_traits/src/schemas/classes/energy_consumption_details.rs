/// <https://schema.org/EnergyConsumptionDetails>
pub trait FindEnergyConsumptionDetailsIds {
	type IdType;
	fn find_energy_consumption_details_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEnergyConsumptionDetailsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_energy_consumption_details_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::ENERGY_CONSUMPTION_DETAILS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::ENERGY_CONSUMPTION_DETAILS_IRI_HTTPS
				}
			})
		}
	}
}
