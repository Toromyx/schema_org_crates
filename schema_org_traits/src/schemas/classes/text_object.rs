/// <https://schema.org/TextObject>
pub trait FindTextObjectIds {
	type IdType;
	/// <https://schema.org/TextObject>
	fn find_text_object_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTextObjectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_text_object_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TEXT_OBJECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TEXT_OBJECT_IRI_HTTPS,
			})
		}
	}
}
