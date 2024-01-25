/// <https://schema.org/Certification>
pub const CERTIFICATION_IRI_HTTP: &str = "http://schema.org/Certification";
/// <https://schema.org/Certification>
pub const CERTIFICATION_IRI_HTTPS: &str = "https://schema.org/Certification";
/// <https://schema.org/Certification>
pub const CERTIFICATION_LABEL: &str = "Certification";
pub struct CertificationIri;
impl PartialEq<&str> for CertificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_IRI_HTTP || *other == CERTIFICATION_IRI_HTTPS
	}
}
impl PartialEq<CertificationIri> for &str {
	fn eq(&self, other: &CertificationIri) -> bool {
		*self == CERTIFICATION_IRI_HTTP || *self == CERTIFICATION_IRI_HTTPS
	}
}
pub struct CertificationIriOrLabel;
impl PartialEq<&str> for CertificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationIri || *other == CERTIFICATION_LABEL
	}
}
impl PartialEq<CertificationIriOrLabel> for &str {
	fn eq(&self, other: &CertificationIriOrLabel) -> bool {
		*self == CertificationIri || *self == CERTIFICATION_LABEL
	}
}
