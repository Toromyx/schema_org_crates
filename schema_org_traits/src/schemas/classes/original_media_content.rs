/// <https://schema.org/OriginalMediaContent>
pub trait FindOriginalMediaContentIds {
	type IdType;
	/// <https://schema.org/OriginalMediaContent>
	fn find_original_media_content_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOriginalMediaContentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_original_media_content_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORIGINAL_MEDIA_CONTENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORIGINAL_MEDIA_CONTENT_IRI_HTTPS,
			})
		}
	}
}
