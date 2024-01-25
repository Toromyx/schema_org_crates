/// <https://schema.org/signOrSymptom>
pub const SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTP: &str = "http://schema.org/signOrSymptom";
/// <https://schema.org/signOrSymptom>
pub const SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/signOrSymptom";
/// <https://schema.org/signOrSymptom>
pub const SIGN_OR_SYMPTOM_PROPERTY_LABEL: &str = "signOrSymptom";
pub struct SignOrSymptomPropertyIri;
impl PartialEq<&str> for SignOrSymptomPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTP || *other == SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SignOrSymptomPropertyIri> for &str {
	fn eq(&self, other: &SignOrSymptomPropertyIri) -> bool {
		*self == SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTP || *self == SIGN_OR_SYMPTOM_PROPERTY_IRI_HTTPS
	}
}
pub struct SignOrSymptomPropertyIriOrLabel;
impl PartialEq<&str> for SignOrSymptomPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SignOrSymptomPropertyIri || *other == SIGN_OR_SYMPTOM_PROPERTY_LABEL
	}
}
impl PartialEq<SignOrSymptomPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SignOrSymptomPropertyIriOrLabel) -> bool {
		*self == SignOrSymptomPropertyIri || *self == SIGN_OR_SYMPTOM_PROPERTY_LABEL
	}
}
