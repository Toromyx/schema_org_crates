/// <https://schema.org/GenericWebPlatform>
pub trait FindGenericWebPlatformIds {
	type IdType;
	/// <https://schema.org/GenericWebPlatform>
	fn find_generic_web_platform_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGenericWebPlatformIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_generic_web_platform_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GENERIC_WEB_PLATFORM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GENERIC_WEB_PLATFORM_IRI_HTTPS,
			})
		}
	}
}
