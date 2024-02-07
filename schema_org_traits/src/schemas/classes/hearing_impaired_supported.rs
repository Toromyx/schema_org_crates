/// <https://schema.org/HearingImpairedSupported>
pub trait FindHearingImpairedSupportedIds {
	type IdType;
	/// <https://schema.org/HearingImpairedSupported>
	fn find_hearing_impaired_supported_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHearingImpairedSupportedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_hearing_impaired_supported_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::HEARING_IMPAIRED_SUPPORTED_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::HEARING_IMPAIRED_SUPPORTED_IRI_HTTPS
				}
			})
		}
	}
}
