/// <https://schema.org/typicalCreditsPerTerm>
pub const TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/typicalCreditsPerTerm";
/// <https://schema.org/typicalCreditsPerTerm>
pub const TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/typicalCreditsPerTerm";
/// <https://schema.org/typicalCreditsPerTerm>
pub const TYPICAL_CREDITS_PER_TERM_PROPERTY_LABEL: &str = "typicalCreditsPerTerm";
pub struct TypicalCreditsPerTermPropertyIri;
impl PartialEq<&str> for TypicalCreditsPerTermPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTP
			|| *other == TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TypicalCreditsPerTermPropertyIri> for &str {
	fn eq(&self, other: &TypicalCreditsPerTermPropertyIri) -> bool {
		*self == TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTP
			|| *self == TYPICAL_CREDITS_PER_TERM_PROPERTY_IRI_HTTPS
	}
}
pub struct TypicalCreditsPerTermPropertyIriOrLabel;
impl PartialEq<&str> for TypicalCreditsPerTermPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypicalCreditsPerTermPropertyIri
			|| *other == TYPICAL_CREDITS_PER_TERM_PROPERTY_LABEL
	}
}
impl PartialEq<TypicalCreditsPerTermPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TypicalCreditsPerTermPropertyIriOrLabel) -> bool {
		*self == TypicalCreditsPerTermPropertyIri
			|| *self == TYPICAL_CREDITS_PER_TERM_PROPERTY_LABEL
	}
}
