/// <https://schema.org/WebAPI>
pub trait FindWebApiIds {
	type IdType;
	/// <https://schema.org/WebAPI>
	fn find_web_api_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWebApiIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_web_api_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEB_API_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WEB_API_IRI_HTTPS,
			})
		}
	}
}
