/// <https://schema.org/suggestedMaxAge>
pub const SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedMaxAge";
/// <https://schema.org/suggestedMaxAge>
pub const SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suggestedMaxAge";
/// <https://schema.org/suggestedMaxAge>
pub const SUGGESTED_MAX_AGE_PROPERTY_LABEL: &str = "suggestedMaxAge";
pub struct SuggestedMaxAgePropertyIri;
impl PartialEq<&str> for SuggestedMaxAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTP
			|| *other == SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedMaxAgePropertyIri> for &str {
	fn eq(&self, other: &SuggestedMaxAgePropertyIri) -> bool {
		*self == SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTP
			|| *self == SUGGESTED_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedMaxAgePropertyIriOrLabel;
impl PartialEq<&str> for SuggestedMaxAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedMaxAgePropertyIri || *other == SUGGESTED_MAX_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedMaxAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedMaxAgePropertyIriOrLabel) -> bool {
		*self == SuggestedMaxAgePropertyIri || *self == SUGGESTED_MAX_AGE_PROPERTY_LABEL
	}
}
