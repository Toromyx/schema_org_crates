/// <https://schema.org/AndroidPlatform>
pub trait FindAndroidPlatformIds {
	type IdType;
	/// <https://schema.org/AndroidPlatform>
	fn find_android_platform_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAndroidPlatformIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_android_platform_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ANDROID_PLATFORM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ANDROID_PLATFORM_IRI_HTTPS,
			})
		}
	}
}
