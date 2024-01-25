/// <https://schema.org/SearchResultsPage>
pub const SEARCH_RESULTS_PAGE_IRI_HTTP: &str = "http://schema.org/SearchResultsPage";
/// <https://schema.org/SearchResultsPage>
pub const SEARCH_RESULTS_PAGE_IRI_HTTPS: &str = "https://schema.org/SearchResultsPage";
/// <https://schema.org/SearchResultsPage>
pub const SEARCH_RESULTS_PAGE_LABEL: &str = "SearchResultsPage";
pub struct SearchResultsPageIri;
impl PartialEq<&str> for SearchResultsPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEARCH_RESULTS_PAGE_IRI_HTTP || *other == SEARCH_RESULTS_PAGE_IRI_HTTPS
	}
}
impl PartialEq<SearchResultsPageIri> for &str {
	fn eq(&self, other: &SearchResultsPageIri) -> bool {
		*self == SEARCH_RESULTS_PAGE_IRI_HTTP || *self == SEARCH_RESULTS_PAGE_IRI_HTTPS
	}
}
pub struct SearchResultsPageIriOrLabel;
impl PartialEq<&str> for SearchResultsPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SearchResultsPageIri || *other == SEARCH_RESULTS_PAGE_LABEL
	}
}
impl PartialEq<SearchResultsPageIriOrLabel> for &str {
	fn eq(&self, other: &SearchResultsPageIriOrLabel) -> bool {
		*self == SearchResultsPageIri || *self == SEARCH_RESULTS_PAGE_LABEL
	}
}
