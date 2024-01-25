/// <https://schema.org/BloodTest>
pub const BLOOD_TEST_IRI_HTTP: &str = "http://schema.org/BloodTest";
/// <https://schema.org/BloodTest>
pub const BLOOD_TEST_IRI_HTTPS: &str = "https://schema.org/BloodTest";
/// <https://schema.org/BloodTest>
pub const BLOOD_TEST_LABEL: &str = "BloodTest";
pub struct BloodTestIri;
impl PartialEq<&str> for BloodTestIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOOD_TEST_IRI_HTTP || *other == BLOOD_TEST_IRI_HTTPS
	}
}
impl PartialEq<BloodTestIri> for &str {
	fn eq(&self, other: &BloodTestIri) -> bool {
		*self == BLOOD_TEST_IRI_HTTP || *self == BLOOD_TEST_IRI_HTTPS
	}
}
pub struct BloodTestIriOrLabel;
impl PartialEq<&str> for BloodTestIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BloodTestIri || *other == BLOOD_TEST_LABEL
	}
}
impl PartialEq<BloodTestIriOrLabel> for &str {
	fn eq(&self, other: &BloodTestIriOrLabel) -> bool {
		*self == BloodTestIri || *self == BLOOD_TEST_LABEL
	}
}
