/// <https://schema.org/CertificationActive>
pub const CERTIFICATION_ACTIVE_IRI_HTTP: &str = "http://schema.org/CertificationActive";
/// <https://schema.org/CertificationActive>
pub const CERTIFICATION_ACTIVE_IRI_HTTPS: &str = "https://schema.org/CertificationActive";
/// <https://schema.org/CertificationActive>
pub const CERTIFICATION_ACTIVE_LABEL: &str = "CertificationActive";
pub struct CertificationActiveIri;
impl PartialEq<&str> for CertificationActiveIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_ACTIVE_IRI_HTTP || *other == CERTIFICATION_ACTIVE_IRI_HTTPS
	}
}
impl PartialEq<CertificationActiveIri> for &str {
	fn eq(&self, other: &CertificationActiveIri) -> bool {
		*self == CERTIFICATION_ACTIVE_IRI_HTTP || *self == CERTIFICATION_ACTIVE_IRI_HTTPS
	}
}
pub struct CertificationActiveIriOrLabel;
impl PartialEq<&str> for CertificationActiveIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationActiveIri || *other == CERTIFICATION_ACTIVE_LABEL
	}
}
impl PartialEq<CertificationActiveIriOrLabel> for &str {
	fn eq(&self, other: &CertificationActiveIriOrLabel) -> bool {
		*self == CertificationActiveIri || *self == CERTIFICATION_ACTIVE_LABEL
	}
}
