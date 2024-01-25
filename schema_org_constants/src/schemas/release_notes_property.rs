/// <https://schema.org/releaseNotes>
pub const RELEASE_NOTES_PROPERTY_IRI_HTTP: &str = "http://schema.org/releaseNotes";
/// <https://schema.org/releaseNotes>
pub const RELEASE_NOTES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/releaseNotes";
/// <https://schema.org/releaseNotes>
pub const RELEASE_NOTES_PROPERTY_LABEL: &str = "releaseNotes";
pub struct ReleaseNotesPropertyIri;
impl PartialEq<&str> for ReleaseNotesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEASE_NOTES_PROPERTY_IRI_HTTP || *other == RELEASE_NOTES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReleaseNotesPropertyIri> for &str {
	fn eq(&self, other: &ReleaseNotesPropertyIri) -> bool {
		*self == RELEASE_NOTES_PROPERTY_IRI_HTTP || *self == RELEASE_NOTES_PROPERTY_IRI_HTTPS
	}
}
pub struct ReleaseNotesPropertyIriOrLabel;
impl PartialEq<&str> for ReleaseNotesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReleaseNotesPropertyIri || *other == RELEASE_NOTES_PROPERTY_LABEL
	}
}
impl PartialEq<ReleaseNotesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReleaseNotesPropertyIriOrLabel) -> bool {
		*self == ReleaseNotesPropertyIri || *self == RELEASE_NOTES_PROPERTY_LABEL
	}
}
