/// <https://schema.org/suggestedAge>
pub const SUGGESTED_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedAge";
/// <https://schema.org/suggestedAge>
pub const SUGGESTED_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suggestedAge";
/// <https://schema.org/suggestedAge>
pub const SUGGESTED_AGE_PROPERTY_LABEL: &str = "suggestedAge";
pub struct SuggestedAgePropertyIri;
impl PartialEq<&str> for SuggestedAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_AGE_PROPERTY_IRI_HTTP || *other == SUGGESTED_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedAgePropertyIri> for &str {
	fn eq(&self, other: &SuggestedAgePropertyIri) -> bool {
		*self == SUGGESTED_AGE_PROPERTY_IRI_HTTP || *self == SUGGESTED_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedAgePropertyIriOrLabel;
impl PartialEq<&str> for SuggestedAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedAgePropertyIri || *other == SUGGESTED_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedAgePropertyIriOrLabel) -> bool {
		*self == SuggestedAgePropertyIri || *self == SUGGESTED_AGE_PROPERTY_LABEL
	}
}
