/// <https://schema.org/assesses>
pub const ASSESSES_PROPERTY_IRI_HTTP: &str = "http://schema.org/assesses";
/// <https://schema.org/assesses>
pub const ASSESSES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/assesses";
/// <https://schema.org/assesses>
pub const ASSESSES_PROPERTY_LABEL: &str = "assesses";
pub struct AssessesPropertyIri;
impl PartialEq<&str> for AssessesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSESSES_PROPERTY_IRI_HTTP || *other == ASSESSES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssessesPropertyIri> for &str {
	fn eq(&self, other: &AssessesPropertyIri) -> bool {
		*self == ASSESSES_PROPERTY_IRI_HTTP || *self == ASSESSES_PROPERTY_IRI_HTTPS
	}
}
pub struct AssessesPropertyIriOrLabel;
impl PartialEq<&str> for AssessesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssessesPropertyIri || *other == ASSESSES_PROPERTY_LABEL
	}
}
impl PartialEq<AssessesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssessesPropertyIriOrLabel) -> bool {
		*self == AssessesPropertyIri || *self == ASSESSES_PROPERTY_LABEL
	}
}
