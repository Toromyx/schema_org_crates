/// <https://schema.org/HealthInsurancePlan>
pub trait FindHealthInsurancePlanIds {
	type IdType;
	/// <https://schema.org/HealthInsurancePlan>
	fn find_health_insurance_plan_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHealthInsurancePlanIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_health_insurance_plan_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HEALTH_INSURANCE_PLAN_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HEALTH_INSURANCE_PLAN_IRI_HTTPS,
			})
		}
	}
}
