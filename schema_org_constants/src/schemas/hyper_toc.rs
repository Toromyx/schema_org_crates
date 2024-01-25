/// <https://schema.org/HyperToc>
pub const HYPER_TOC_IRI_HTTP: &str = "http://schema.org/HyperToc";
/// <https://schema.org/HyperToc>
pub const HYPER_TOC_IRI_HTTPS: &str = "https://schema.org/HyperToc";
/// <https://schema.org/HyperToc>
pub const HYPER_TOC_LABEL: &str = "HyperToc";
pub struct HyperTocIri;
impl PartialEq<&str> for HyperTocIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HYPER_TOC_IRI_HTTP || *other == HYPER_TOC_IRI_HTTPS
	}
}
impl PartialEq<HyperTocIri> for &str {
	fn eq(&self, other: &HyperTocIri) -> bool {
		*self == HYPER_TOC_IRI_HTTP || *self == HYPER_TOC_IRI_HTTPS
	}
}
pub struct HyperTocIriOrLabel;
impl PartialEq<&str> for HyperTocIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HyperTocIri || *other == HYPER_TOC_LABEL
	}
}
impl PartialEq<HyperTocIriOrLabel> for &str {
	fn eq(&self, other: &HyperTocIriOrLabel) -> bool {
		*self == HyperTocIri || *self == HYPER_TOC_LABEL
	}
}
