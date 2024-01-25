/// <https://schema.org/signDetected>
pub const SIGN_DETECTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/signDetected";
/// <https://schema.org/signDetected>
pub const SIGN_DETECTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/signDetected";
/// <https://schema.org/signDetected>
pub const SIGN_DETECTED_PROPERTY_LABEL: &str = "signDetected";
pub struct SignDetectedPropertyIri;
impl PartialEq<&str> for SignDetectedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIGN_DETECTED_PROPERTY_IRI_HTTP || *other == SIGN_DETECTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SignDetectedPropertyIri> for &str {
	fn eq(&self, other: &SignDetectedPropertyIri) -> bool {
		*self == SIGN_DETECTED_PROPERTY_IRI_HTTP || *self == SIGN_DETECTED_PROPERTY_IRI_HTTPS
	}
}
pub struct SignDetectedPropertyIriOrLabel;
impl PartialEq<&str> for SignDetectedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SignDetectedPropertyIri || *other == SIGN_DETECTED_PROPERTY_LABEL
	}
}
impl PartialEq<SignDetectedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SignDetectedPropertyIriOrLabel) -> bool {
		*self == SignDetectedPropertyIri || *self == SIGN_DETECTED_PROPERTY_LABEL
	}
}
