/// <https://schema.org/HealthPlanFormulary>
pub trait FindHealthPlanFormularyIds {
	type IdType;
	/// <https://schema.org/HealthPlanFormulary>
	fn find_health_plan_formulary_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHealthPlanFormularyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_health_plan_formulary_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HEALTH_PLAN_FORMULARY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HEALTH_PLAN_FORMULARY_IRI_HTTPS,
			})
		}
	}
}
