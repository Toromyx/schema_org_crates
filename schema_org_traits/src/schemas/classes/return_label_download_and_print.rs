/// <https://schema.org/ReturnLabelDownloadAndPrint>
pub trait FindReturnLabelDownloadAndPrintIds {
	type IdType;
	/// <https://schema.org/ReturnLabelDownloadAndPrint>
	fn find_return_label_download_and_print_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnLabelDownloadAndPrintIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_label_download_and_print_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTPS
				}
			})
		}
	}
}
