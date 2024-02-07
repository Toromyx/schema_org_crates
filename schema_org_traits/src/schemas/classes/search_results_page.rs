/// <https://schema.org/SearchResultsPage>
pub trait FindSearchResultsPageIds {
	type IdType;
	/// <https://schema.org/SearchResultsPage>
	fn find_search_results_page_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSearchResultsPageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_search_results_page_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SEARCH_RESULTS_PAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SEARCH_RESULTS_PAGE_IRI_HTTPS,
			})
		}
	}
}
