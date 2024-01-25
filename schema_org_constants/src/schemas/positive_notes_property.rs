/// <https://schema.org/positiveNotes>
pub const POSITIVE_NOTES_PROPERTY_IRI_HTTP: &str = "http://schema.org/positiveNotes";
/// <https://schema.org/positiveNotes>
pub const POSITIVE_NOTES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/positiveNotes";
/// <https://schema.org/positiveNotes>
pub const POSITIVE_NOTES_PROPERTY_LABEL: &str = "positiveNotes";
pub struct PositiveNotesPropertyIri;
impl PartialEq<&str> for PositiveNotesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSITIVE_NOTES_PROPERTY_IRI_HTTP || *other == POSITIVE_NOTES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PositiveNotesPropertyIri> for &str {
	fn eq(&self, other: &PositiveNotesPropertyIri) -> bool {
		*self == POSITIVE_NOTES_PROPERTY_IRI_HTTP || *self == POSITIVE_NOTES_PROPERTY_IRI_HTTPS
	}
}
pub struct PositiveNotesPropertyIriOrLabel;
impl PartialEq<&str> for PositiveNotesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PositiveNotesPropertyIri || *other == POSITIVE_NOTES_PROPERTY_LABEL
	}
}
impl PartialEq<PositiveNotesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PositiveNotesPropertyIriOrLabel) -> bool {
		*self == PositiveNotesPropertyIri || *self == POSITIVE_NOTES_PROPERTY_LABEL
	}
}
