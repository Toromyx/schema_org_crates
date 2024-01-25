/// <https://schema.org/tocContinuation>
pub const TOC_CONTINUATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/tocContinuation";
/// <https://schema.org/tocContinuation>
pub const TOC_CONTINUATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tocContinuation";
/// <https://schema.org/tocContinuation>
pub const TOC_CONTINUATION_PROPERTY_LABEL: &str = "tocContinuation";
pub struct TocContinuationPropertyIri;
impl PartialEq<&str> for TocContinuationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOC_CONTINUATION_PROPERTY_IRI_HTTP
			|| *other == TOC_CONTINUATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TocContinuationPropertyIri> for &str {
	fn eq(&self, other: &TocContinuationPropertyIri) -> bool {
		*self == TOC_CONTINUATION_PROPERTY_IRI_HTTP || *self == TOC_CONTINUATION_PROPERTY_IRI_HTTPS
	}
}
pub struct TocContinuationPropertyIriOrLabel;
impl PartialEq<&str> for TocContinuationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TocContinuationPropertyIri || *other == TOC_CONTINUATION_PROPERTY_LABEL
	}
}
impl PartialEq<TocContinuationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TocContinuationPropertyIriOrLabel) -> bool {
		*self == TocContinuationPropertyIri || *self == TOC_CONTINUATION_PROPERTY_LABEL
	}
}
