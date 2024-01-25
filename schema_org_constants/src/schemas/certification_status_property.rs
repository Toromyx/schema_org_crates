/// <https://schema.org/certificationStatus>
pub const CERTIFICATION_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/certificationStatus";
/// <https://schema.org/certificationStatus>
pub const CERTIFICATION_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/certificationStatus";
/// <https://schema.org/certificationStatus>
pub const CERTIFICATION_STATUS_PROPERTY_LABEL: &str = "certificationStatus";
pub struct CertificationStatusPropertyIri;
impl PartialEq<&str> for CertificationStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_STATUS_PROPERTY_IRI_HTTP
			|| *other == CERTIFICATION_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CertificationStatusPropertyIri> for &str {
	fn eq(&self, other: &CertificationStatusPropertyIri) -> bool {
		*self == CERTIFICATION_STATUS_PROPERTY_IRI_HTTP
			|| *self == CERTIFICATION_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct CertificationStatusPropertyIriOrLabel;
impl PartialEq<&str> for CertificationStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationStatusPropertyIri || *other == CERTIFICATION_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<CertificationStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CertificationStatusPropertyIriOrLabel) -> bool {
		*self == CertificationStatusPropertyIri || *self == CERTIFICATION_STATUS_PROPERTY_LABEL
	}
}
