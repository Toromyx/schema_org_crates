/// <https://schema.org/CertificationInactive>
pub const CERTIFICATION_INACTIVE_IRI_HTTP: &str = "http://schema.org/CertificationInactive";
/// <https://schema.org/CertificationInactive>
pub const CERTIFICATION_INACTIVE_IRI_HTTPS: &str = "https://schema.org/CertificationInactive";
/// <https://schema.org/CertificationInactive>
pub const CERTIFICATION_INACTIVE_LABEL: &str = "CertificationInactive";
pub struct CertificationInactiveIri;
impl PartialEq<&str> for CertificationInactiveIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_INACTIVE_IRI_HTTP || *other == CERTIFICATION_INACTIVE_IRI_HTTPS
	}
}
impl PartialEq<CertificationInactiveIri> for &str {
	fn eq(&self, other: &CertificationInactiveIri) -> bool {
		*self == CERTIFICATION_INACTIVE_IRI_HTTP || *self == CERTIFICATION_INACTIVE_IRI_HTTPS
	}
}
pub struct CertificationInactiveIriOrLabel;
impl PartialEq<&str> for CertificationInactiveIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationInactiveIri || *other == CERTIFICATION_INACTIVE_LABEL
	}
}
impl PartialEq<CertificationInactiveIriOrLabel> for &str {
	fn eq(&self, other: &CertificationInactiveIriOrLabel) -> bool {
		*self == CertificationInactiveIri || *self == CERTIFICATION_INACTIVE_LABEL
	}
}
