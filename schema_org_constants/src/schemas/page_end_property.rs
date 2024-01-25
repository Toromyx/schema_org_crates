/// <https://schema.org/pageEnd>
pub const PAGE_END_PROPERTY_IRI_HTTP: &str = "http://schema.org/pageEnd";
/// <https://schema.org/pageEnd>
pub const PAGE_END_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pageEnd";
/// <https://schema.org/pageEnd>
pub const PAGE_END_PROPERTY_LABEL: &str = "pageEnd";
pub struct PageEndPropertyIri;
impl PartialEq<&str> for PageEndPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAGE_END_PROPERTY_IRI_HTTP || *other == PAGE_END_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PageEndPropertyIri> for &str {
	fn eq(&self, other: &PageEndPropertyIri) -> bool {
		*self == PAGE_END_PROPERTY_IRI_HTTP || *self == PAGE_END_PROPERTY_IRI_HTTPS
	}
}
pub struct PageEndPropertyIriOrLabel;
impl PartialEq<&str> for PageEndPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PageEndPropertyIri || *other == PAGE_END_PROPERTY_LABEL
	}
}
impl PartialEq<PageEndPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PageEndPropertyIriOrLabel) -> bool {
		*self == PageEndPropertyIri || *self == PAGE_END_PROPERTY_LABEL
	}
}
