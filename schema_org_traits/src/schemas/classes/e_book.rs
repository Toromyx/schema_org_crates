/// <https://schema.org/EBook>
pub trait FindEBookIds {
	type IdType;
	/// <https://schema.org/EBook>
	fn find_e_book_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEBookIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_e_book_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::E_BOOK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::E_BOOK_IRI_HTTPS,
			})
		}
	}
}
