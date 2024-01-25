/// <https://schema.org/recordingOf>
pub const RECORDING_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/recordingOf";
/// <https://schema.org/recordingOf>
pub const RECORDING_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recordingOf";
/// <https://schema.org/recordingOf>
pub const RECORDING_OF_PROPERTY_LABEL: &str = "recordingOf";
pub struct RecordingOfPropertyIri;
impl PartialEq<&str> for RecordingOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECORDING_OF_PROPERTY_IRI_HTTP || *other == RECORDING_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecordingOfPropertyIri> for &str {
	fn eq(&self, other: &RecordingOfPropertyIri) -> bool {
		*self == RECORDING_OF_PROPERTY_IRI_HTTP || *self == RECORDING_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct RecordingOfPropertyIriOrLabel;
impl PartialEq<&str> for RecordingOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecordingOfPropertyIri || *other == RECORDING_OF_PROPERTY_LABEL
	}
}
impl PartialEq<RecordingOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecordingOfPropertyIriOrLabel) -> bool {
		*self == RecordingOfPropertyIri || *self == RECORDING_OF_PROPERTY_LABEL
	}
}
