/// <https://schema.org/Emergency>
pub const EMERGENCY_IRI_HTTP: &str = "http://schema.org/Emergency";
/// <https://schema.org/Emergency>
pub const EMERGENCY_IRI_HTTPS: &str = "https://schema.org/Emergency";
/// <https://schema.org/Emergency>
pub const EMERGENCY_LABEL: &str = "Emergency";
pub struct EmergencyIri;
impl PartialEq<&str> for EmergencyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMERGENCY_IRI_HTTP || *other == EMERGENCY_IRI_HTTPS
	}
}
impl PartialEq<EmergencyIri> for &str {
	fn eq(&self, other: &EmergencyIri) -> bool {
		*self == EMERGENCY_IRI_HTTP || *self == EMERGENCY_IRI_HTTPS
	}
}
pub struct EmergencyIriOrLabel;
impl PartialEq<&str> for EmergencyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmergencyIri || *other == EMERGENCY_LABEL
	}
}
impl PartialEq<EmergencyIriOrLabel> for &str {
	fn eq(&self, other: &EmergencyIriOrLabel) -> bool {
		*self == EmergencyIri || *self == EMERGENCY_LABEL
	}
}
