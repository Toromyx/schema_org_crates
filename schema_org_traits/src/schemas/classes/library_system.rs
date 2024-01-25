/// <https://schema.org/LibrarySystem>
pub trait FindLibrarySystemIds {
	type IdType;
	fn find_library_system_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLibrarySystemIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_library_system_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LIBRARY_SYSTEM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LIBRARY_SYSTEM_IRI_HTTPS,
			})
		}
	}
}
