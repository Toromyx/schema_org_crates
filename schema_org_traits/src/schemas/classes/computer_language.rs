/// <https://schema.org/ComputerLanguage>
pub trait FindComputerLanguageIds {
	type IdType;
	/// <https://schema.org/ComputerLanguage>
	fn find_computer_language_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindComputerLanguageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_computer_language_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COMPUTER_LANGUAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::COMPUTER_LANGUAGE_IRI_HTTPS,
			})
		}
	}
}
