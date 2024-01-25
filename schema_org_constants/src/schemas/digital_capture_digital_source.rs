/// <https://schema.org/DigitalCaptureDigitalSource>
pub const DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/DigitalCaptureDigitalSource";
/// <https://schema.org/DigitalCaptureDigitalSource>
pub const DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/DigitalCaptureDigitalSource";
/// <https://schema.org/DigitalCaptureDigitalSource>
pub const DIGITAL_CAPTURE_DIGITAL_SOURCE_LABEL: &str = "DigitalCaptureDigitalSource";
pub struct DigitalCaptureDigitalSourceIri;
impl PartialEq<&str> for DigitalCaptureDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTP
			|| *other == DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<DigitalCaptureDigitalSourceIri> for &str {
	fn eq(&self, other: &DigitalCaptureDigitalSourceIri) -> bool {
		*self == DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTP
			|| *self == DIGITAL_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct DigitalCaptureDigitalSourceIriOrLabel;
impl PartialEq<&str> for DigitalCaptureDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalCaptureDigitalSourceIri || *other == DIGITAL_CAPTURE_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<DigitalCaptureDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &DigitalCaptureDigitalSourceIriOrLabel) -> bool {
		*self == DigitalCaptureDigitalSourceIri || *self == DIGITAL_CAPTURE_DIGITAL_SOURCE_LABEL
	}
}
