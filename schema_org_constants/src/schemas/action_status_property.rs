/// <https://schema.org/actionStatus>
pub const ACTION_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/actionStatus";
/// <https://schema.org/actionStatus>
pub const ACTION_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actionStatus";
/// <https://schema.org/actionStatus>
pub const ACTION_STATUS_PROPERTY_LABEL: &str = "actionStatus";
pub struct ActionStatusPropertyIri;
impl PartialEq<&str> for ActionStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_STATUS_PROPERTY_IRI_HTTP || *other == ACTION_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionStatusPropertyIri> for &str {
	fn eq(&self, other: &ActionStatusPropertyIri) -> bool {
		*self == ACTION_STATUS_PROPERTY_IRI_HTTP || *self == ACTION_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionStatusPropertyIriOrLabel;
impl PartialEq<&str> for ActionStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionStatusPropertyIri || *other == ACTION_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<ActionStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionStatusPropertyIriOrLabel) -> bool {
		*self == ActionStatusPropertyIri || *self == ACTION_STATUS_PROPERTY_LABEL
	}
}
