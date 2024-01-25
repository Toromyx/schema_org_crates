/// <https://schema.org/DigitalPlatformEnumeration>
pub trait FindDigitalPlatformEnumerationIds {
	type IdType;
	fn find_digital_platform_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDigitalPlatformEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_digital_platform_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::DIGITAL_PLATFORM_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::DIGITAL_PLATFORM_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
