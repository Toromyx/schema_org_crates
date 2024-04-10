/// <https://schema.org/NoteDigitalDocument>
pub trait FindNoteDigitalDocumentIds {
	type IdType;
	/// <https://schema.org/NoteDigitalDocument>
	fn find_note_digital_document_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNoteDigitalDocumentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_note_digital_document_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NOTE_DIGITAL_DOCUMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NOTE_DIGITAL_DOCUMENT_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNoteDigitalDocumentIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_note_digital_document_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NOTE_DIGITAL_DOCUMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NOTE_DIGITAL_DOCUMENT_IRI_HTTPS,
			})
		}
	}
}
