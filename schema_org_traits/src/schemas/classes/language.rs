/// <https://schema.org/Language>
pub trait FindLanguageIds {
	type IdType;
	/// <https://schema.org/Language>
	fn find_language_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLanguageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_language_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LANGUAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LANGUAGE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLanguageIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_language_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LANGUAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LANGUAGE_IRI_HTTPS,
			})
		}
	}
}
