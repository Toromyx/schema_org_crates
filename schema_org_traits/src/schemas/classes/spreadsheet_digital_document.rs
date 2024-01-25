/// <https://schema.org/SpreadsheetDigitalDocument>
pub trait FindSpreadsheetDigitalDocumentIds {
	type IdType;
	fn find_spreadsheet_digital_document_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSpreadsheetDigitalDocumentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_spreadsheet_digital_document_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTPS
				}
			})
		}
	}
}
