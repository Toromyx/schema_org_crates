/// <https://schema.org/certificationIdentification>
pub const CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/certificationIdentification";
/// <https://schema.org/certificationIdentification>
pub const CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/certificationIdentification";
/// <https://schema.org/certificationIdentification>
pub const CERTIFICATION_IDENTIFICATION_PROPERTY_LABEL: &str = "certificationIdentification";
pub struct CertificationIdentificationPropertyIri;
impl PartialEq<&str> for CertificationIdentificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTP
			|| *other == CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CertificationIdentificationPropertyIri> for &str {
	fn eq(&self, other: &CertificationIdentificationPropertyIri) -> bool {
		*self == CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTP
			|| *self == CERTIFICATION_IDENTIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct CertificationIdentificationPropertyIriOrLabel;
impl PartialEq<&str> for CertificationIdentificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationIdentificationPropertyIri
			|| *other == CERTIFICATION_IDENTIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<CertificationIdentificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CertificationIdentificationPropertyIriOrLabel) -> bool {
		*self == CertificationIdentificationPropertyIri
			|| *self == CERTIFICATION_IDENTIFICATION_PROPERTY_LABEL
	}
}
