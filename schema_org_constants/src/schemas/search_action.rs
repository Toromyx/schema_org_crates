/// <https://schema.org/SearchAction>
pub const SEARCH_ACTION_IRI_HTTP: &str = "http://schema.org/SearchAction";
/// <https://schema.org/SearchAction>
pub const SEARCH_ACTION_IRI_HTTPS: &str = "https://schema.org/SearchAction";
/// <https://schema.org/SearchAction>
pub const SEARCH_ACTION_LABEL: &str = "SearchAction";
pub struct SearchActionIri;
impl PartialEq<&str> for SearchActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEARCH_ACTION_IRI_HTTP || *other == SEARCH_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SearchActionIri> for &str {
	fn eq(&self, other: &SearchActionIri) -> bool {
		*self == SEARCH_ACTION_IRI_HTTP || *self == SEARCH_ACTION_IRI_HTTPS
	}
}
pub struct SearchActionIriOrLabel;
impl PartialEq<&str> for SearchActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SearchActionIri || *other == SEARCH_ACTION_LABEL
	}
}
impl PartialEq<SearchActionIriOrLabel> for &str {
	fn eq(&self, other: &SearchActionIriOrLabel) -> bool {
		*self == SearchActionIri || *self == SEARCH_ACTION_LABEL
	}
}
