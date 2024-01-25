/// <https://schema.org/suggestedMinAge>
pub const SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedMinAge";
/// <https://schema.org/suggestedMinAge>
pub const SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suggestedMinAge";
/// <https://schema.org/suggestedMinAge>
pub const SUGGESTED_MIN_AGE_PROPERTY_LABEL: &str = "suggestedMinAge";
pub struct SuggestedMinAgePropertyIri;
impl PartialEq<&str> for SuggestedMinAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTP
			|| *other == SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedMinAgePropertyIri> for &str {
	fn eq(&self, other: &SuggestedMinAgePropertyIri) -> bool {
		*self == SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTP
			|| *self == SUGGESTED_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedMinAgePropertyIriOrLabel;
impl PartialEq<&str> for SuggestedMinAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedMinAgePropertyIri || *other == SUGGESTED_MIN_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedMinAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedMinAgePropertyIriOrLabel) -> bool {
		*self == SuggestedMinAgePropertyIri || *self == SUGGESTED_MIN_AGE_PROPERTY_LABEL
	}
}
