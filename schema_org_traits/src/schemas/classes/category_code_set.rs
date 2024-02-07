/// <https://schema.org/CategoryCodeSet>
pub trait FindCategoryCodeSetIds {
	type IdType;
	/// <https://schema.org/CategoryCodeSet>
	fn find_category_code_set_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCategoryCodeSetIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_category_code_set_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CATEGORY_CODE_SET_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CATEGORY_CODE_SET_IRI_HTTPS,
			})
		}
	}
}
