/// <https://schema.org/expectedPrognosis>
pub const EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTP: &str = "http://schema.org/expectedPrognosis";
/// <https://schema.org/expectedPrognosis>
pub const EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/expectedPrognosis";
/// <https://schema.org/expectedPrognosis>
pub const EXPECTED_PROGNOSIS_PROPERTY_LABEL: &str = "expectedPrognosis";
pub struct ExpectedPrognosisPropertyIri;
impl PartialEq<&str> for ExpectedPrognosisPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTP
			|| *other == EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpectedPrognosisPropertyIri> for &str {
	fn eq(&self, other: &ExpectedPrognosisPropertyIri) -> bool {
		*self == EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTP
			|| *self == EXPECTED_PROGNOSIS_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpectedPrognosisPropertyIriOrLabel;
impl PartialEq<&str> for ExpectedPrognosisPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpectedPrognosisPropertyIri || *other == EXPECTED_PROGNOSIS_PROPERTY_LABEL
	}
}
impl PartialEq<ExpectedPrognosisPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpectedPrognosisPropertyIriOrLabel) -> bool {
		*self == ExpectedPrognosisPropertyIri || *self == EXPECTED_PROGNOSIS_PROPERTY_LABEL
	}
}
