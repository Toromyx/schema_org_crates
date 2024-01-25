/// <https://schema.org/CompositeCaptureDigitalSource>
pub const COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/CompositeCaptureDigitalSource";
/// <https://schema.org/CompositeCaptureDigitalSource>
pub const COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/CompositeCaptureDigitalSource";
/// <https://schema.org/CompositeCaptureDigitalSource>
pub const COMPOSITE_CAPTURE_DIGITAL_SOURCE_LABEL: &str = "CompositeCaptureDigitalSource";
pub struct CompositeCaptureDigitalSourceIri;
impl PartialEq<&str> for CompositeCaptureDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTP
			|| *other == COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<CompositeCaptureDigitalSourceIri> for &str {
	fn eq(&self, other: &CompositeCaptureDigitalSourceIri) -> bool {
		*self == COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTP
			|| *self == COMPOSITE_CAPTURE_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct CompositeCaptureDigitalSourceIriOrLabel;
impl PartialEq<&str> for CompositeCaptureDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompositeCaptureDigitalSourceIri
			|| *other == COMPOSITE_CAPTURE_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<CompositeCaptureDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &CompositeCaptureDigitalSourceIriOrLabel) -> bool {
		*self == CompositeCaptureDigitalSourceIri || *self == COMPOSITE_CAPTURE_DIGITAL_SOURCE_LABEL
	}
}
