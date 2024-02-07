/// <https://schema.org/MinorHumanEditsDigitalSource>
pub trait FindMinorHumanEditsDigitalSourceIds {
	type IdType;
	/// <https://schema.org/MinorHumanEditsDigitalSource>
	fn find_minor_human_edits_digital_source_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMinorHumanEditsDigitalSourceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_minor_human_edits_digital_source_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTPS
				}
			})
		}
	}
}
