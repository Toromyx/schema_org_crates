/// <https://schema.org/DesktopWebPlatform>
pub trait FindDesktopWebPlatformIds {
	type IdType;
	fn find_desktop_web_platform_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDesktopWebPlatformIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_desktop_web_platform_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DESKTOP_WEB_PLATFORM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DESKTOP_WEB_PLATFORM_IRI_HTTPS,
			})
		}
	}
}
