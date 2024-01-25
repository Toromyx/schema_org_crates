/// <https://schema.org/securityScreening>
pub const SECURITY_SCREENING_PROPERTY_IRI_HTTP: &str = "http://schema.org/securityScreening";
/// <https://schema.org/securityScreening>
pub const SECURITY_SCREENING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/securityScreening";
/// <https://schema.org/securityScreening>
pub const SECURITY_SCREENING_PROPERTY_LABEL: &str = "securityScreening";
pub struct SecurityScreeningPropertyIri;
impl PartialEq<&str> for SecurityScreeningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SECURITY_SCREENING_PROPERTY_IRI_HTTP
			|| *other == SECURITY_SCREENING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SecurityScreeningPropertyIri> for &str {
	fn eq(&self, other: &SecurityScreeningPropertyIri) -> bool {
		*self == SECURITY_SCREENING_PROPERTY_IRI_HTTP
			|| *self == SECURITY_SCREENING_PROPERTY_IRI_HTTPS
	}
}
pub struct SecurityScreeningPropertyIriOrLabel;
impl PartialEq<&str> for SecurityScreeningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SecurityScreeningPropertyIri || *other == SECURITY_SCREENING_PROPERTY_LABEL
	}
}
impl PartialEq<SecurityScreeningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SecurityScreeningPropertyIriOrLabel) -> bool {
		*self == SecurityScreeningPropertyIri || *self == SECURITY_SCREENING_PROPERTY_LABEL
	}
}
