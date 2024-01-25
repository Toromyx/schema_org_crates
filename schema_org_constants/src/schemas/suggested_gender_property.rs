/// <https://schema.org/suggestedGender>
pub const SUGGESTED_GENDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedGender";
/// <https://schema.org/suggestedGender>
pub const SUGGESTED_GENDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suggestedGender";
/// <https://schema.org/suggestedGender>
pub const SUGGESTED_GENDER_PROPERTY_LABEL: &str = "suggestedGender";
pub struct SuggestedGenderPropertyIri;
impl PartialEq<&str> for SuggestedGenderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_GENDER_PROPERTY_IRI_HTTP
			|| *other == SUGGESTED_GENDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedGenderPropertyIri> for &str {
	fn eq(&self, other: &SuggestedGenderPropertyIri) -> bool {
		*self == SUGGESTED_GENDER_PROPERTY_IRI_HTTP || *self == SUGGESTED_GENDER_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedGenderPropertyIriOrLabel;
impl PartialEq<&str> for SuggestedGenderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedGenderPropertyIri || *other == SUGGESTED_GENDER_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedGenderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedGenderPropertyIriOrLabel) -> bool {
		*self == SuggestedGenderPropertyIri || *self == SUGGESTED_GENDER_PROPERTY_LABEL
	}
}
