/// <https://schema.org/requiredGender>
pub const REQUIRED_GENDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiredGender";
/// <https://schema.org/requiredGender>
pub const REQUIRED_GENDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requiredGender";
/// <https://schema.org/requiredGender>
pub const REQUIRED_GENDER_PROPERTY_LABEL: &str = "requiredGender";
pub struct RequiredGenderPropertyIri;
impl PartialEq<&str> for RequiredGenderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRED_GENDER_PROPERTY_IRI_HTTP || *other == REQUIRED_GENDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiredGenderPropertyIri> for &str {
	fn eq(&self, other: &RequiredGenderPropertyIri) -> bool {
		*self == REQUIRED_GENDER_PROPERTY_IRI_HTTP || *self == REQUIRED_GENDER_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiredGenderPropertyIriOrLabel;
impl PartialEq<&str> for RequiredGenderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiredGenderPropertyIri || *other == REQUIRED_GENDER_PROPERTY_LABEL
	}
}
impl PartialEq<RequiredGenderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiredGenderPropertyIriOrLabel) -> bool {
		*self == RequiredGenderPropertyIri || *self == REQUIRED_GENDER_PROPERTY_LABEL
	}
}
