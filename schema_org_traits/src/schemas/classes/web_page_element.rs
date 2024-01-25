/// <https://schema.org/WebPageElement>
pub trait FindWebPageElementIds {
	type IdType;
	fn find_web_page_element_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWebPageElementIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_web_page_element_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEB_PAGE_ELEMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WEB_PAGE_ELEMENT_IRI_HTTPS,
			})
		}
	}
}
