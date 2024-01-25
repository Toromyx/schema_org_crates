/// <https://schema.org/recordLabel>
pub const RECORD_LABEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/recordLabel";
/// <https://schema.org/recordLabel>
pub const RECORD_LABEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recordLabel";
/// <https://schema.org/recordLabel>
pub const RECORD_LABEL_PROPERTY_LABEL: &str = "recordLabel";
pub struct RecordLabelPropertyIri;
impl PartialEq<&str> for RecordLabelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECORD_LABEL_PROPERTY_IRI_HTTP || *other == RECORD_LABEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecordLabelPropertyIri> for &str {
	fn eq(&self, other: &RecordLabelPropertyIri) -> bool {
		*self == RECORD_LABEL_PROPERTY_IRI_HTTP || *self == RECORD_LABEL_PROPERTY_IRI_HTTPS
	}
}
pub struct RecordLabelPropertyIriOrLabel;
impl PartialEq<&str> for RecordLabelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecordLabelPropertyIri || *other == RECORD_LABEL_PROPERTY_LABEL
	}
}
impl PartialEq<RecordLabelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecordLabelPropertyIriOrLabel) -> bool {
		*self == RecordLabelPropertyIri || *self == RECORD_LABEL_PROPERTY_LABEL
	}
}
