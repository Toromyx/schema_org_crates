/// <https://schema.org/VirtualRecordingDigitalSource>
pub const VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/VirtualRecordingDigitalSource";
/// <https://schema.org/VirtualRecordingDigitalSource>
pub const VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/VirtualRecordingDigitalSource";
/// <https://schema.org/VirtualRecordingDigitalSource>
pub const VIRTUAL_RECORDING_DIGITAL_SOURCE_LABEL: &str = "VirtualRecordingDigitalSource";
pub struct VirtualRecordingDigitalSourceIri;
impl PartialEq<&str> for VirtualRecordingDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTP
			|| *other == VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<VirtualRecordingDigitalSourceIri> for &str {
	fn eq(&self, other: &VirtualRecordingDigitalSourceIri) -> bool {
		*self == VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTP
			|| *self == VIRTUAL_RECORDING_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct VirtualRecordingDigitalSourceIriOrLabel;
impl PartialEq<&str> for VirtualRecordingDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VirtualRecordingDigitalSourceIri
			|| *other == VIRTUAL_RECORDING_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<VirtualRecordingDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &VirtualRecordingDigitalSourceIriOrLabel) -> bool {
		*self == VirtualRecordingDigitalSourceIri || *self == VIRTUAL_RECORDING_DIGITAL_SOURCE_LABEL
	}
}
