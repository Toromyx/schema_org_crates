/// <https://schema.org/CssSelectorType>
pub trait FindCssSelectorTypeIds {
	type IdType;
	/// <https://schema.org/CssSelectorType>
	fn find_css_selector_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCssSelectorTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_css_selector_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CSS_SELECTOR_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CSS_SELECTOR_TYPE_IRI_HTTPS,
			})
		}
	}
}
