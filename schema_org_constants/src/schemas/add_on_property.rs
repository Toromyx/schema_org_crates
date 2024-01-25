/// <https://schema.org/addOn>
pub const ADD_ON_PROPERTY_IRI_HTTP: &str = "http://schema.org/addOn";
/// <https://schema.org/addOn>
pub const ADD_ON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/addOn";
/// <https://schema.org/addOn>
pub const ADD_ON_PROPERTY_LABEL: &str = "addOn";
pub struct AddOnPropertyIri;
impl PartialEq<&str> for AddOnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADD_ON_PROPERTY_IRI_HTTP || *other == ADD_ON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AddOnPropertyIri> for &str {
	fn eq(&self, other: &AddOnPropertyIri) -> bool {
		*self == ADD_ON_PROPERTY_IRI_HTTP || *self == ADD_ON_PROPERTY_IRI_HTTPS
	}
}
pub struct AddOnPropertyIriOrLabel;
impl PartialEq<&str> for AddOnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddOnPropertyIri || *other == ADD_ON_PROPERTY_LABEL
	}
}
impl PartialEq<AddOnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AddOnPropertyIriOrLabel) -> bool {
		*self == AddOnPropertyIri || *self == ADD_ON_PROPERTY_LABEL
	}
}
