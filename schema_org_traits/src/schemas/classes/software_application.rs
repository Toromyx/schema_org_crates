/// <https://schema.org/SoftwareApplication>
pub trait FindSoftwareApplicationIds {
	type IdType;
	/// <https://schema.org/SoftwareApplication>
	fn find_software_application_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSoftwareApplicationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_software_application_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SOFTWARE_APPLICATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SOFTWARE_APPLICATION_IRI_HTTPS,
			})
		}
	}
}
