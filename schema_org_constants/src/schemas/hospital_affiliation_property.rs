/// <https://schema.org/hospitalAffiliation>
pub const HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hospitalAffiliation";
/// <https://schema.org/hospitalAffiliation>
pub const HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hospitalAffiliation";
/// <https://schema.org/hospitalAffiliation>
pub const HOSPITAL_AFFILIATION_PROPERTY_LABEL: &str = "hospitalAffiliation";
pub struct HospitalAffiliationPropertyIri;
impl PartialEq<&str> for HospitalAffiliationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTP
			|| *other == HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HospitalAffiliationPropertyIri> for &str {
	fn eq(&self, other: &HospitalAffiliationPropertyIri) -> bool {
		*self == HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTP
			|| *self == HOSPITAL_AFFILIATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HospitalAffiliationPropertyIriOrLabel;
impl PartialEq<&str> for HospitalAffiliationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HospitalAffiliationPropertyIri || *other == HOSPITAL_AFFILIATION_PROPERTY_LABEL
	}
}
impl PartialEq<HospitalAffiliationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HospitalAffiliationPropertyIriOrLabel) -> bool {
		*self == HospitalAffiliationPropertyIri || *self == HOSPITAL_AFFILIATION_PROPERTY_LABEL
	}
}
