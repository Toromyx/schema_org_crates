/// <https://schema.org/PathologyTest>
pub trait FindPathologyTestIds {
	type IdType;
	/// <https://schema.org/PathologyTest>
	fn find_pathology_test_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPathologyTestIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_pathology_test_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PATHOLOGY_TEST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PATHOLOGY_TEST_IRI_HTTPS,
			})
		}
	}
}
