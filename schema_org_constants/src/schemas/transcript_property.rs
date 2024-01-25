/// <https://schema.org/transcript>
pub const TRANSCRIPT_PROPERTY_IRI_HTTP: &str = "http://schema.org/transcript";
/// <https://schema.org/transcript>
pub const TRANSCRIPT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/transcript";
/// <https://schema.org/transcript>
pub const TRANSCRIPT_PROPERTY_LABEL: &str = "transcript";
pub struct TranscriptPropertyIri;
impl PartialEq<&str> for TranscriptPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSCRIPT_PROPERTY_IRI_HTTP || *other == TRANSCRIPT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TranscriptPropertyIri> for &str {
	fn eq(&self, other: &TranscriptPropertyIri) -> bool {
		*self == TRANSCRIPT_PROPERTY_IRI_HTTP || *self == TRANSCRIPT_PROPERTY_IRI_HTTPS
	}
}
pub struct TranscriptPropertyIriOrLabel;
impl PartialEq<&str> for TranscriptPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TranscriptPropertyIri || *other == TRANSCRIPT_PROPERTY_LABEL
	}
}
impl PartialEq<TranscriptPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TranscriptPropertyIriOrLabel) -> bool {
		*self == TranscriptPropertyIri || *self == TRANSCRIPT_PROPERTY_LABEL
	}
}
