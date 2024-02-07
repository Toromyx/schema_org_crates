/// <https://schema.org/APIReference>
pub trait FindApiReferenceIds {
	type IdType;
	/// <https://schema.org/APIReference>
	fn find_api_reference_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindApiReferenceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_api_reference_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::API_REFERENCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::API_REFERENCE_IRI_HTTPS,
			})
		}
	}
}
