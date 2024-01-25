/// <https://schema.org/pageStart>
pub const PAGE_START_PROPERTY_IRI_HTTP: &str = "http://schema.org/pageStart";
/// <https://schema.org/pageStart>
pub const PAGE_START_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pageStart";
/// <https://schema.org/pageStart>
pub const PAGE_START_PROPERTY_LABEL: &str = "pageStart";
pub struct PageStartPropertyIri;
impl PartialEq<&str> for PageStartPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAGE_START_PROPERTY_IRI_HTTP || *other == PAGE_START_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PageStartPropertyIri> for &str {
	fn eq(&self, other: &PageStartPropertyIri) -> bool {
		*self == PAGE_START_PROPERTY_IRI_HTTP || *self == PAGE_START_PROPERTY_IRI_HTTPS
	}
}
pub struct PageStartPropertyIriOrLabel;
impl PartialEq<&str> for PageStartPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PageStartPropertyIri || *other == PAGE_START_PROPERTY_LABEL
	}
}
impl PartialEq<PageStartPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PageStartPropertyIriOrLabel) -> bool {
		*self == PageStartPropertyIri || *self == PAGE_START_PROPERTY_LABEL
	}
}
