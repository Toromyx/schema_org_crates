/// <https://schema.org/PrintDigitalSource>
pub trait FindPrintDigitalSourceIds {
	type IdType;
	/// <https://schema.org/PrintDigitalSource>
	fn find_print_digital_source_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPrintDigitalSourceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_print_digital_source_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRINT_DIGITAL_SOURCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRINT_DIGITAL_SOURCE_IRI_HTTPS,
			})
		}
	}
}
