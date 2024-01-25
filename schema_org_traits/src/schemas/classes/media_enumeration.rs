/// <https://schema.org/MediaEnumeration>
pub trait FindMediaEnumerationIds {
	type IdType;
	fn find_media_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMediaEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_media_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDIA_ENUMERATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDIA_ENUMERATION_IRI_HTTPS,
			})
		}
	}
}
