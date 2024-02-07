/// <https://schema.org/DiagnosticLab>
pub trait FindDiagnosticLabIds {
	type IdType;
	/// <https://schema.org/DiagnosticLab>
	fn find_diagnostic_lab_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDiagnosticLabIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_diagnostic_lab_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DIAGNOSTIC_LAB_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DIAGNOSTIC_LAB_IRI_HTTPS,
			})
		}
	}
}
