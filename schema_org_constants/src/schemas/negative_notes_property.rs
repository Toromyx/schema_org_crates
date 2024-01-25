/// <https://schema.org/negativeNotes>
pub const NEGATIVE_NOTES_PROPERTY_IRI_HTTP: &str = "http://schema.org/negativeNotes";
/// <https://schema.org/negativeNotes>
pub const NEGATIVE_NOTES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/negativeNotes";
/// <https://schema.org/negativeNotes>
pub const NEGATIVE_NOTES_PROPERTY_LABEL: &str = "negativeNotes";
pub struct NegativeNotesPropertyIri;
impl PartialEq<&str> for NegativeNotesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEGATIVE_NOTES_PROPERTY_IRI_HTTP || *other == NEGATIVE_NOTES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NegativeNotesPropertyIri> for &str {
	fn eq(&self, other: &NegativeNotesPropertyIri) -> bool {
		*self == NEGATIVE_NOTES_PROPERTY_IRI_HTTP || *self == NEGATIVE_NOTES_PROPERTY_IRI_HTTPS
	}
}
pub struct NegativeNotesPropertyIriOrLabel;
impl PartialEq<&str> for NegativeNotesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NegativeNotesPropertyIri || *other == NEGATIVE_NOTES_PROPERTY_LABEL
	}
}
impl PartialEq<NegativeNotesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NegativeNotesPropertyIriOrLabel) -> bool {
		*self == NegativeNotesPropertyIri || *self == NEGATIVE_NOTES_PROPERTY_LABEL
	}
}
