/// <https://schema.org/HealthPlanNetwork>
pub trait FindHealthPlanNetworkIds {
	type IdType;
	/// <https://schema.org/HealthPlanNetwork>
	fn find_health_plan_network_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHealthPlanNetworkIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_health_plan_network_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HEALTH_PLAN_NETWORK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HEALTH_PLAN_NETWORK_IRI_HTTPS,
			})
		}
	}
}
