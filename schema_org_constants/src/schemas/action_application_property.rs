/// <https://schema.org/actionApplication>
pub const ACTION_APPLICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/actionApplication";
/// <https://schema.org/actionApplication>
pub const ACTION_APPLICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actionApplication";
/// <https://schema.org/actionApplication>
pub const ACTION_APPLICATION_PROPERTY_LABEL: &str = "actionApplication";
pub struct ActionApplicationPropertyIri;
impl PartialEq<&str> for ActionApplicationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_APPLICATION_PROPERTY_IRI_HTTP
			|| *other == ACTION_APPLICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionApplicationPropertyIri> for &str {
	fn eq(&self, other: &ActionApplicationPropertyIri) -> bool {
		*self == ACTION_APPLICATION_PROPERTY_IRI_HTTP
			|| *self == ACTION_APPLICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionApplicationPropertyIriOrLabel;
impl PartialEq<&str> for ActionApplicationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionApplicationPropertyIri || *other == ACTION_APPLICATION_PROPERTY_LABEL
	}
}
impl PartialEq<ActionApplicationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionApplicationPropertyIriOrLabel) -> bool {
		*self == ActionApplicationPropertyIri || *self == ACTION_APPLICATION_PROPERTY_LABEL
	}
}
