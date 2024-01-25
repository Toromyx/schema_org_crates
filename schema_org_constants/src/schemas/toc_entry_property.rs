/// <https://schema.org/tocEntry>
pub const TOC_ENTRY_PROPERTY_IRI_HTTP: &str = "http://schema.org/tocEntry";
/// <https://schema.org/tocEntry>
pub const TOC_ENTRY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tocEntry";
/// <https://schema.org/tocEntry>
pub const TOC_ENTRY_PROPERTY_LABEL: &str = "tocEntry";
pub struct TocEntryPropertyIri;
impl PartialEq<&str> for TocEntryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOC_ENTRY_PROPERTY_IRI_HTTP || *other == TOC_ENTRY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TocEntryPropertyIri> for &str {
	fn eq(&self, other: &TocEntryPropertyIri) -> bool {
		*self == TOC_ENTRY_PROPERTY_IRI_HTTP || *self == TOC_ENTRY_PROPERTY_IRI_HTTPS
	}
}
pub struct TocEntryPropertyIriOrLabel;
impl PartialEq<&str> for TocEntryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TocEntryPropertyIri || *other == TOC_ENTRY_PROPERTY_LABEL
	}
}
impl PartialEq<TocEntryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TocEntryPropertyIriOrLabel) -> bool {
		*self == TocEntryPropertyIri || *self == TOC_ENTRY_PROPERTY_LABEL
	}
}
