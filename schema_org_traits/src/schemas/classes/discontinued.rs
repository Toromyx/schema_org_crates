/// <https://schema.org/Discontinued>
pub trait FindDiscontinuedIds {
	type IdType;
	/// <https://schema.org/Discontinued>
	fn find_discontinued_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDiscontinuedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_discontinued_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DISCONTINUED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DISCONTINUED_IRI_HTTPS,
			})
		}
	}
}
