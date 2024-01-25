/// <https://schema.org/editEIDR>
pub const EDIT_EIDR_PROPERTY_IRI_HTTP: &str = "http://schema.org/editEIDR";
/// <https://schema.org/editEIDR>
pub const EDIT_EIDR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/editEIDR";
/// <https://schema.org/editEIDR>
pub const EDIT_EIDR_PROPERTY_LABEL: &str = "editEIDR";
pub struct EditEidrPropertyIri;
impl PartialEq<&str> for EditEidrPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDIT_EIDR_PROPERTY_IRI_HTTP || *other == EDIT_EIDR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EditEidrPropertyIri> for &str {
	fn eq(&self, other: &EditEidrPropertyIri) -> bool {
		*self == EDIT_EIDR_PROPERTY_IRI_HTTP || *self == EDIT_EIDR_PROPERTY_IRI_HTTPS
	}
}
pub struct EditEidrPropertyIriOrLabel;
impl PartialEq<&str> for EditEidrPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EditEidrPropertyIri || *other == EDIT_EIDR_PROPERTY_LABEL
	}
}
impl PartialEq<EditEidrPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EditEidrPropertyIriOrLabel) -> bool {
		*self == EditEidrPropertyIri || *self == EDIT_EIDR_PROPERTY_LABEL
	}
}
