/// <https://schema.org/BookFormatType>
pub trait FindBookFormatTypeIds {
	type IdType;
	fn find_book_format_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBookFormatTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_book_format_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BOOK_FORMAT_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BOOK_FORMAT_TYPE_IRI_HTTPS,
			})
		}
	}
}
