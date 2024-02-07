/// <https://schema.org/Newspaper>
pub trait FindNewspaperIds {
	type IdType;
	/// <https://schema.org/Newspaper>
	fn find_newspaper_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNewspaperIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_newspaper_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NEWSPAPER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NEWSPAPER_IRI_HTTPS,
			})
		}
	}
}
