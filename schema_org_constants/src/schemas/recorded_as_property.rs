/// <https://schema.org/recordedAs>
pub const RECORDED_AS_PROPERTY_IRI_HTTP: &str = "http://schema.org/recordedAs";
/// <https://schema.org/recordedAs>
pub const RECORDED_AS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recordedAs";
/// <https://schema.org/recordedAs>
pub const RECORDED_AS_PROPERTY_LABEL: &str = "recordedAs";
pub struct RecordedAsPropertyIri;
impl PartialEq<&str> for RecordedAsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECORDED_AS_PROPERTY_IRI_HTTP || *other == RECORDED_AS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecordedAsPropertyIri> for &str {
	fn eq(&self, other: &RecordedAsPropertyIri) -> bool {
		*self == RECORDED_AS_PROPERTY_IRI_HTTP || *self == RECORDED_AS_PROPERTY_IRI_HTTPS
	}
}
pub struct RecordedAsPropertyIriOrLabel;
impl PartialEq<&str> for RecordedAsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecordedAsPropertyIri || *other == RECORDED_AS_PROPERTY_LABEL
	}
}
impl PartialEq<RecordedAsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecordedAsPropertyIriOrLabel) -> bool {
		*self == RecordedAsPropertyIri || *self == RECORDED_AS_PROPERTY_LABEL
	}
}
