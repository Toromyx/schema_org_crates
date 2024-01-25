/// <https://schema.org/organizer>
pub const ORGANIZER_PROPERTY_IRI_HTTP: &str = "http://schema.org/organizer";
/// <https://schema.org/organizer>
pub const ORGANIZER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/organizer";
/// <https://schema.org/organizer>
pub const ORGANIZER_PROPERTY_LABEL: &str = "organizer";
pub struct OrganizerPropertyIri;
impl PartialEq<&str> for OrganizerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORGANIZER_PROPERTY_IRI_HTTP || *other == ORGANIZER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrganizerPropertyIri> for &str {
	fn eq(&self, other: &OrganizerPropertyIri) -> bool {
		*self == ORGANIZER_PROPERTY_IRI_HTTP || *self == ORGANIZER_PROPERTY_IRI_HTTPS
	}
}
pub struct OrganizerPropertyIriOrLabel;
impl PartialEq<&str> for OrganizerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrganizerPropertyIri || *other == ORGANIZER_PROPERTY_LABEL
	}
}
impl PartialEq<OrganizerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrganizerPropertyIriOrLabel) -> bool {
		*self == OrganizerPropertyIri || *self == ORGANIZER_PROPERTY_LABEL
	}
}
