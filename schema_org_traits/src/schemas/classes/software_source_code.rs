/// <https://schema.org/SoftwareSourceCode>
pub trait FindSoftwareSourceCodeIds {
	type IdType;
	/// <https://schema.org/SoftwareSourceCode>
	fn find_software_source_code_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSoftwareSourceCodeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_software_source_code_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SOFTWARE_SOURCE_CODE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SOFTWARE_SOURCE_CODE_IRI_HTTPS,
			})
		}
	}
}
