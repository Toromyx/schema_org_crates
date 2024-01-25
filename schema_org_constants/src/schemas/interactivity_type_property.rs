/// <https://schema.org/interactivityType>
pub const INTERACTIVITY_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactivityType";
/// <https://schema.org/interactivityType>
pub const INTERACTIVITY_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interactivityType";
/// <https://schema.org/interactivityType>
pub const INTERACTIVITY_TYPE_PROPERTY_LABEL: &str = "interactivityType";
pub struct InteractivityTypePropertyIri;
impl PartialEq<&str> for InteractivityTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTIVITY_TYPE_PROPERTY_IRI_HTTP
			|| *other == INTERACTIVITY_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractivityTypePropertyIri> for &str {
	fn eq(&self, other: &InteractivityTypePropertyIri) -> bool {
		*self == INTERACTIVITY_TYPE_PROPERTY_IRI_HTTP
			|| *self == INTERACTIVITY_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractivityTypePropertyIriOrLabel;
impl PartialEq<&str> for InteractivityTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractivityTypePropertyIri || *other == INTERACTIVITY_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<InteractivityTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractivityTypePropertyIriOrLabel) -> bool {
		*self == InteractivityTypePropertyIri || *self == INTERACTIVITY_TYPE_PROPERTY_LABEL
	}
}
