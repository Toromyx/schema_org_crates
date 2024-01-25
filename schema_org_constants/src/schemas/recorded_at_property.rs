/// <https://schema.org/recordedAt>
pub const RECORDED_AT_PROPERTY_IRI_HTTP: &str = "http://schema.org/recordedAt";
/// <https://schema.org/recordedAt>
pub const RECORDED_AT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recordedAt";
/// <https://schema.org/recordedAt>
pub const RECORDED_AT_PROPERTY_LABEL: &str = "recordedAt";
pub struct RecordedAtPropertyIri;
impl PartialEq<&str> for RecordedAtPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECORDED_AT_PROPERTY_IRI_HTTP || *other == RECORDED_AT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecordedAtPropertyIri> for &str {
	fn eq(&self, other: &RecordedAtPropertyIri) -> bool {
		*self == RECORDED_AT_PROPERTY_IRI_HTTP || *self == RECORDED_AT_PROPERTY_IRI_HTTPS
	}
}
pub struct RecordedAtPropertyIriOrLabel;
impl PartialEq<&str> for RecordedAtPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecordedAtPropertyIri || *other == RECORDED_AT_PROPERTY_LABEL
	}
}
impl PartialEq<RecordedAtPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecordedAtPropertyIriOrLabel) -> bool {
		*self == RecordedAtPropertyIri || *self == RECORDED_AT_PROPERTY_LABEL
	}
}
