/// <https://schema.org/GettingAccessHealthAspect>
pub trait FindGettingAccessHealthAspectIds {
	type IdType;
	/// <https://schema.org/GettingAccessHealthAspect>
	fn find_getting_access_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGettingAccessHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_getting_access_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::GETTING_ACCESS_HEALTH_ASPECT_IRI_HTTPS
				}
			})
		}
	}
}
