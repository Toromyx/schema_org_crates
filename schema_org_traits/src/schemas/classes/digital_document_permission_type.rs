/// <https://schema.org/DigitalDocumentPermissionType>
pub trait FindDigitalDocumentPermissionTypeIds {
	type IdType;
	fn find_digital_document_permission_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDigitalDocumentPermissionTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_digital_document_permission_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTPS
				}
			})
		}
	}
}
