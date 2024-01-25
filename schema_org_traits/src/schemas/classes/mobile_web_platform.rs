/// <https://schema.org/MobileWebPlatform>
pub trait FindMobileWebPlatformIds {
	type IdType;
	fn find_mobile_web_platform_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMobileWebPlatformIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_mobile_web_platform_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MOBILE_WEB_PLATFORM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MOBILE_WEB_PLATFORM_IRI_HTTPS,
			})
		}
	}
}
