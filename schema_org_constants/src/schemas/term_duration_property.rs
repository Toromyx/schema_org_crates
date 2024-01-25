/// <https://schema.org/termDuration>
pub const TERM_DURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/termDuration";
/// <https://schema.org/termDuration>
pub const TERM_DURATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/termDuration";
/// <https://schema.org/termDuration>
pub const TERM_DURATION_PROPERTY_LABEL: &str = "termDuration";
pub struct TermDurationPropertyIri;
impl PartialEq<&str> for TermDurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TERM_DURATION_PROPERTY_IRI_HTTP || *other == TERM_DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TermDurationPropertyIri> for &str {
	fn eq(&self, other: &TermDurationPropertyIri) -> bool {
		*self == TERM_DURATION_PROPERTY_IRI_HTTP || *self == TERM_DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct TermDurationPropertyIriOrLabel;
impl PartialEq<&str> for TermDurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TermDurationPropertyIri || *other == TERM_DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<TermDurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TermDurationPropertyIriOrLabel) -> bool {
		*self == TermDurationPropertyIri || *self == TERM_DURATION_PROPERTY_LABEL
	}
}
