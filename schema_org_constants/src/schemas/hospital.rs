/// <https://schema.org/Hospital>
pub const HOSPITAL_IRI_HTTP: &str = "http://schema.org/Hospital";
/// <https://schema.org/Hospital>
pub const HOSPITAL_IRI_HTTPS: &str = "https://schema.org/Hospital";
/// <https://schema.org/Hospital>
pub const HOSPITAL_LABEL: &str = "Hospital";
pub struct HospitalIri;
impl PartialEq<&str> for HospitalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOSPITAL_IRI_HTTP || *other == HOSPITAL_IRI_HTTPS
	}
}
impl PartialEq<HospitalIri> for &str {
	fn eq(&self, other: &HospitalIri) -> bool {
		*self == HOSPITAL_IRI_HTTP || *self == HOSPITAL_IRI_HTTPS
	}
}
pub struct HospitalIriOrLabel;
impl PartialEq<&str> for HospitalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HospitalIri || *other == HOSPITAL_LABEL
	}
}
impl PartialEq<HospitalIriOrLabel> for &str {
	fn eq(&self, other: &HospitalIriOrLabel) -> bool {
		*self == HospitalIri || *self == HOSPITAL_LABEL
	}
}
