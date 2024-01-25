/// <https://schema.org/FloorPlan>
pub trait FindFloorPlanIds {
	type IdType;
	fn find_floor_plan_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFloorPlanIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_floor_plan_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FLOOR_PLAN_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FLOOR_PLAN_IRI_HTTPS,
			})
		}
	}
}
