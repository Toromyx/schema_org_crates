/// <https://schema.org/termsPerYear>
pub const TERMS_PER_YEAR_PROPERTY_IRI_HTTP: &str = "http://schema.org/termsPerYear";
/// <https://schema.org/termsPerYear>
pub const TERMS_PER_YEAR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/termsPerYear";
/// <https://schema.org/termsPerYear>
pub const TERMS_PER_YEAR_PROPERTY_LABEL: &str = "termsPerYear";
pub struct TermsPerYearPropertyIri;
impl PartialEq<&str> for TermsPerYearPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TERMS_PER_YEAR_PROPERTY_IRI_HTTP || *other == TERMS_PER_YEAR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TermsPerYearPropertyIri> for &str {
	fn eq(&self, other: &TermsPerYearPropertyIri) -> bool {
		*self == TERMS_PER_YEAR_PROPERTY_IRI_HTTP || *self == TERMS_PER_YEAR_PROPERTY_IRI_HTTPS
	}
}
pub struct TermsPerYearPropertyIriOrLabel;
impl PartialEq<&str> for TermsPerYearPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TermsPerYearPropertyIri || *other == TERMS_PER_YEAR_PROPERTY_LABEL
	}
}
impl PartialEq<TermsPerYearPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TermsPerYearPropertyIriOrLabel) -> bool {
		*self == TermsPerYearPropertyIri || *self == TERMS_PER_YEAR_PROPERTY_LABEL
	}
}
