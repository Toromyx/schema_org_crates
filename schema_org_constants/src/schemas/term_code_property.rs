/// <https://schema.org/termCode>
pub const TERM_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/termCode";
/// <https://schema.org/termCode>
pub const TERM_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/termCode";
/// <https://schema.org/termCode>
pub const TERM_CODE_PROPERTY_LABEL: &str = "termCode";
pub struct TermCodePropertyIri;
impl PartialEq<&str> for TermCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TERM_CODE_PROPERTY_IRI_HTTP || *other == TERM_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TermCodePropertyIri> for &str {
	fn eq(&self, other: &TermCodePropertyIri) -> bool {
		*self == TERM_CODE_PROPERTY_IRI_HTTP || *self == TERM_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct TermCodePropertyIriOrLabel;
impl PartialEq<&str> for TermCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TermCodePropertyIri || *other == TERM_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<TermCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TermCodePropertyIriOrLabel) -> bool {
		*self == TermCodePropertyIri || *self == TERM_CODE_PROPERTY_LABEL
	}
}
