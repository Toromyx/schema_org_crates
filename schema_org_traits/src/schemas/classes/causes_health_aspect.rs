/// <https://schema.org/CausesHealthAspect>
pub trait FindCausesHealthAspectIds {
	type IdType;
	/// <https://schema.org/CausesHealthAspect>
	fn find_causes_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCausesHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_causes_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CAUSES_HEALTH_ASPECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CAUSES_HEALTH_ASPECT_IRI_HTTPS,
			})
		}
	}
}
