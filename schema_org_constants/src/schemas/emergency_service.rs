/// <https://schema.org/EmergencyService>
pub const EMERGENCY_SERVICE_IRI_HTTP: &str = "http://schema.org/EmergencyService";
/// <https://schema.org/EmergencyService>
pub const EMERGENCY_SERVICE_IRI_HTTPS: &str = "https://schema.org/EmergencyService";
/// <https://schema.org/EmergencyService>
pub const EMERGENCY_SERVICE_LABEL: &str = "EmergencyService";
pub struct EmergencyServiceIri;
impl PartialEq<&str> for EmergencyServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMERGENCY_SERVICE_IRI_HTTP || *other == EMERGENCY_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<EmergencyServiceIri> for &str {
	fn eq(&self, other: &EmergencyServiceIri) -> bool {
		*self == EMERGENCY_SERVICE_IRI_HTTP || *self == EMERGENCY_SERVICE_IRI_HTTPS
	}
}
pub struct EmergencyServiceIriOrLabel;
impl PartialEq<&str> for EmergencyServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmergencyServiceIri || *other == EMERGENCY_SERVICE_LABEL
	}
}
impl PartialEq<EmergencyServiceIriOrLabel> for &str {
	fn eq(&self, other: &EmergencyServiceIriOrLabel) -> bool {
		*self == EmergencyServiceIri || *self == EMERGENCY_SERVICE_LABEL
	}
}
