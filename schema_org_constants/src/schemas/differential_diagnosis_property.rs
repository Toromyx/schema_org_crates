/// <https://schema.org/differentialDiagnosis>
pub const DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/differentialDiagnosis";
/// <https://schema.org/differentialDiagnosis>
pub const DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/differentialDiagnosis";
/// <https://schema.org/differentialDiagnosis>
pub const DIFFERENTIAL_DIAGNOSIS_PROPERTY_LABEL: &str = "differentialDiagnosis";
pub struct DifferentialDiagnosisPropertyIri;
impl PartialEq<&str> for DifferentialDiagnosisPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTP
			|| *other == DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DifferentialDiagnosisPropertyIri> for &str {
	fn eq(&self, other: &DifferentialDiagnosisPropertyIri) -> bool {
		*self == DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTP
			|| *self == DIFFERENTIAL_DIAGNOSIS_PROPERTY_IRI_HTTPS
	}
}
pub struct DifferentialDiagnosisPropertyIriOrLabel;
impl PartialEq<&str> for DifferentialDiagnosisPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DifferentialDiagnosisPropertyIri
			|| *other == DIFFERENTIAL_DIAGNOSIS_PROPERTY_LABEL
	}
}
impl PartialEq<DifferentialDiagnosisPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DifferentialDiagnosisPropertyIriOrLabel) -> bool {
		*self == DifferentialDiagnosisPropertyIri || *self == DIFFERENTIAL_DIAGNOSIS_PROPERTY_LABEL
	}
}
