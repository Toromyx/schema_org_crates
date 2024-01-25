/// <https://schema.org/AddAction>
pub const ADD_ACTION_IRI_HTTP: &str = "http://schema.org/AddAction";
/// <https://schema.org/AddAction>
pub const ADD_ACTION_IRI_HTTPS: &str = "https://schema.org/AddAction";
/// <https://schema.org/AddAction>
pub const ADD_ACTION_LABEL: &str = "AddAction";
pub struct AddActionIri;
impl PartialEq<&str> for AddActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADD_ACTION_IRI_HTTP || *other == ADD_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AddActionIri> for &str {
	fn eq(&self, other: &AddActionIri) -> bool {
		*self == ADD_ACTION_IRI_HTTP || *self == ADD_ACTION_IRI_HTTPS
	}
}
pub struct AddActionIriOrLabel;
impl PartialEq<&str> for AddActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddActionIri || *other == ADD_ACTION_LABEL
	}
}
impl PartialEq<AddActionIriOrLabel> for &str {
	fn eq(&self, other: &AddActionIriOrLabel) -> bool {
		*self == AddActionIri || *self == ADD_ACTION_LABEL
	}
}
