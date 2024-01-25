/// <https://schema.org/DigitalDocumentPermission>
pub trait FindDigitalDocumentPermissionIds {
	type IdType;
	fn find_digital_document_permission_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDigitalDocumentPermissionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_digital_document_permission_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::DIGITAL_DOCUMENT_PERMISSION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::DIGITAL_DOCUMENT_PERMISSION_IRI_HTTPS
				}
			})
		}
	}
}
