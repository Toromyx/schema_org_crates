/// <https://schema.org/Ultrasound>
pub const ULTRASOUND_IRI_HTTP: &str = "http://schema.org/Ultrasound";
/// <https://schema.org/Ultrasound>
pub const ULTRASOUND_IRI_HTTPS: &str = "https://schema.org/Ultrasound";
/// <https://schema.org/Ultrasound>
pub const ULTRASOUND_LABEL: &str = "Ultrasound";
pub struct UltrasoundIri;
impl PartialEq<&str> for UltrasoundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ULTRASOUND_IRI_HTTP || *other == ULTRASOUND_IRI_HTTPS
	}
}
impl PartialEq<UltrasoundIri> for &str {
	fn eq(&self, other: &UltrasoundIri) -> bool {
		*self == ULTRASOUND_IRI_HTTP || *self == ULTRASOUND_IRI_HTTPS
	}
}
pub struct UltrasoundIriOrLabel;
impl PartialEq<&str> for UltrasoundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UltrasoundIri || *other == ULTRASOUND_LABEL
	}
}
impl PartialEq<UltrasoundIriOrLabel> for &str {
	fn eq(&self, other: &UltrasoundIriOrLabel) -> bool {
		*self == UltrasoundIri || *self == ULTRASOUND_LABEL
	}
}
