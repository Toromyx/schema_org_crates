/// <https://schema.org/CertificationStatusEnumeration>
pub const CERTIFICATION_STATUS_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/CertificationStatusEnumeration";
/// <https://schema.org/CertificationStatusEnumeration>
pub const CERTIFICATION_STATUS_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/CertificationStatusEnumeration";
/// <https://schema.org/CertificationStatusEnumeration>
pub const CERTIFICATION_STATUS_ENUMERATION_LABEL: &str = "CertificationStatusEnumeration";
pub struct CertificationStatusEnumerationIri;
impl PartialEq<&str> for CertificationStatusEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CERTIFICATION_STATUS_ENUMERATION_IRI_HTTP
			|| *other == CERTIFICATION_STATUS_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<CertificationStatusEnumerationIri> for &str {
	fn eq(&self, other: &CertificationStatusEnumerationIri) -> bool {
		*self == CERTIFICATION_STATUS_ENUMERATION_IRI_HTTP
			|| *self == CERTIFICATION_STATUS_ENUMERATION_IRI_HTTPS
	}
}
pub struct CertificationStatusEnumerationIriOrLabel;
impl PartialEq<&str> for CertificationStatusEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CertificationStatusEnumerationIri
			|| *other == CERTIFICATION_STATUS_ENUMERATION_LABEL
	}
}
impl PartialEq<CertificationStatusEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &CertificationStatusEnumerationIriOrLabel) -> bool {
		*self == CertificationStatusEnumerationIri
			|| *self == CERTIFICATION_STATUS_ENUMERATION_LABEL
	}
}
