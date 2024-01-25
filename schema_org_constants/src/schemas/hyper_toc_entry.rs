/// <https://schema.org/HyperTocEntry>
pub const HYPER_TOC_ENTRY_IRI_HTTP: &str = "http://schema.org/HyperTocEntry";
/// <https://schema.org/HyperTocEntry>
pub const HYPER_TOC_ENTRY_IRI_HTTPS: &str = "https://schema.org/HyperTocEntry";
/// <https://schema.org/HyperTocEntry>
pub const HYPER_TOC_ENTRY_LABEL: &str = "HyperTocEntry";
pub struct HyperTocEntryIri;
impl PartialEq<&str> for HyperTocEntryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HYPER_TOC_ENTRY_IRI_HTTP || *other == HYPER_TOC_ENTRY_IRI_HTTPS
	}
}
impl PartialEq<HyperTocEntryIri> for &str {
	fn eq(&self, other: &HyperTocEntryIri) -> bool {
		*self == HYPER_TOC_ENTRY_IRI_HTTP || *self == HYPER_TOC_ENTRY_IRI_HTTPS
	}
}
pub struct HyperTocEntryIriOrLabel;
impl PartialEq<&str> for HyperTocEntryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HyperTocEntryIri || *other == HYPER_TOC_ENTRY_LABEL
	}
}
impl PartialEq<HyperTocEntryIriOrLabel> for &str {
	fn eq(&self, other: &HyperTocEntryIriOrLabel) -> bool {
		*self == HyperTocEntryIri || *self == HYPER_TOC_ENTRY_LABEL
	}
}
