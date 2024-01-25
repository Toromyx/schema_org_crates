/// <https://schema.org/MedicalTest>
pub const MEDICAL_TEST_IRI_HTTP: &str = "http://schema.org/MedicalTest";
/// <https://schema.org/MedicalTest>
pub const MEDICAL_TEST_IRI_HTTPS: &str = "https://schema.org/MedicalTest";
/// <https://schema.org/MedicalTest>
pub const MEDICAL_TEST_LABEL: &str = "MedicalTest";
pub struct MedicalTestIri;
impl PartialEq<&str> for MedicalTestIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_TEST_IRI_HTTP || *other == MEDICAL_TEST_IRI_HTTPS
	}
}
impl PartialEq<MedicalTestIri> for &str {
	fn eq(&self, other: &MedicalTestIri) -> bool {
		*self == MEDICAL_TEST_IRI_HTTP || *self == MEDICAL_TEST_IRI_HTTPS
	}
}
pub struct MedicalTestIriOrLabel;
impl PartialEq<&str> for MedicalTestIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalTestIri || *other == MEDICAL_TEST_LABEL
	}
}
impl PartialEq<MedicalTestIriOrLabel> for &str {
	fn eq(&self, other: &MedicalTestIriOrLabel) -> bool {
		*self == MedicalTestIri || *self == MEDICAL_TEST_LABEL
	}
}
