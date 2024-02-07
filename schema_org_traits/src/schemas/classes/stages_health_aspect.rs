/// <https://schema.org/StagesHealthAspect>
pub trait FindStagesHealthAspectIds {
	type IdType;
	/// <https://schema.org/StagesHealthAspect>
	fn find_stages_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindStagesHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_stages_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::STAGES_HEALTH_ASPECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::STAGES_HEALTH_ASPECT_IRI_HTTPS,
			})
		}
	}
}
