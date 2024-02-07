/// <https://schema.org/RisksOrComplicationsHealthAspect>
pub trait FindRisksOrComplicationsHealthAspectIds {
	type IdType;
	/// <https://schema.org/RisksOrComplicationsHealthAspect>
	fn find_risks_or_complications_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRisksOrComplicationsHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_risks_or_complications_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::RISKS_OR_COMPLICATIONS_HEALTH_ASPECT_IRI_HTTPS
				}
			})
		}
	}
}
