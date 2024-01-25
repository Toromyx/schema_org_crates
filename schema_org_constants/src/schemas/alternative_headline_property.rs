/// <https://schema.org/alternativeHeadline>
pub const ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/alternativeHeadline";
/// <https://schema.org/alternativeHeadline>
pub const ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alternativeHeadline";
/// <https://schema.org/alternativeHeadline>
pub const ALTERNATIVE_HEADLINE_PROPERTY_LABEL: &str = "alternativeHeadline";
pub struct AlternativeHeadlinePropertyIri;
impl PartialEq<&str> for AlternativeHeadlinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTP
			|| *other == ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlternativeHeadlinePropertyIri> for &str {
	fn eq(&self, other: &AlternativeHeadlinePropertyIri) -> bool {
		*self == ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTP
			|| *self == ALTERNATIVE_HEADLINE_PROPERTY_IRI_HTTPS
	}
}
pub struct AlternativeHeadlinePropertyIriOrLabel;
impl PartialEq<&str> for AlternativeHeadlinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlternativeHeadlinePropertyIri || *other == ALTERNATIVE_HEADLINE_PROPERTY_LABEL
	}
}
impl PartialEq<AlternativeHeadlinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlternativeHeadlinePropertyIriOrLabel) -> bool {
		*self == AlternativeHeadlinePropertyIri || *self == ALTERNATIVE_HEADLINE_PROPERTY_LABEL
	}
}
