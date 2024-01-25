/// <https://schema.org/HealthAspectEnumeration>
pub trait FindHealthAspectEnumerationIds {
	type IdType;
	fn find_health_aspect_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHealthAspectEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_health_aspect_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::HEALTH_ASPECT_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::HEALTH_ASPECT_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
