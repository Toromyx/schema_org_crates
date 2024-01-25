/// <https://schema.org/PresentationDigitalDocument>
pub trait FindPresentationDigitalDocumentIds {
	type IdType;
	fn find_presentation_digital_document_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPresentationDigitalDocumentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_presentation_digital_document_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTPS
				}
			})
		}
	}
}
