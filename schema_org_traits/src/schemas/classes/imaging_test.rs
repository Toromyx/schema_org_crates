/// <https://schema.org/ImagingTest>
pub trait FindImagingTestIds {
	type IdType;
	/// <https://schema.org/ImagingTest>
	fn find_imaging_test_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindImagingTestIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_imaging_test_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::IMAGING_TEST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::IMAGING_TEST_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindImagingTestIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_imaging_test_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::IMAGING_TEST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::IMAGING_TEST_IRI_HTTPS,
			})
		}
	}
}
