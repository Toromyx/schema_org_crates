/// <https://schema.org/isAcceptingNewPatients>
pub const IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isAcceptingNewPatients";
/// <https://schema.org/isAcceptingNewPatients>
pub const IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isAcceptingNewPatients";
/// <https://schema.org/isAcceptingNewPatients>
pub const IS_ACCEPTING_NEW_PATIENTS_PROPERTY_LABEL: &str = "isAcceptingNewPatients";
pub struct IsAcceptingNewPatientsPropertyIri;
impl PartialEq<&str> for IsAcceptingNewPatientsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTP
			|| *other == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsAcceptingNewPatientsPropertyIri> for &str {
	fn eq(&self, other: &IsAcceptingNewPatientsPropertyIri) -> bool {
		*self == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTP
			|| *self == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct IsAcceptingNewPatientsPropertyIriOrLabel;
impl PartialEq<&str> for IsAcceptingNewPatientsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsAcceptingNewPatientsPropertyIri
			|| *other == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_LABEL
	}
}
impl PartialEq<IsAcceptingNewPatientsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsAcceptingNewPatientsPropertyIriOrLabel) -> bool {
		*self == IsAcceptingNewPatientsPropertyIri
			|| *self == IS_ACCEPTING_NEW_PATIENTS_PROPERTY_LABEL
	}
}
