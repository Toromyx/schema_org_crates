/// <https://schema.org/actionOption>
pub const ACTION_OPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/actionOption";
/// <https://schema.org/actionOption>
pub const ACTION_OPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actionOption";
/// <https://schema.org/actionOption>
pub const ACTION_OPTION_PROPERTY_LABEL: &str = "actionOption";
pub struct ActionOptionPropertyIri;
impl PartialEq<&str> for ActionOptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_OPTION_PROPERTY_IRI_HTTP || *other == ACTION_OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionOptionPropertyIri> for &str {
	fn eq(&self, other: &ActionOptionPropertyIri) -> bool {
		*self == ACTION_OPTION_PROPERTY_IRI_HTTP || *self == ACTION_OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionOptionPropertyIriOrLabel;
impl PartialEq<&str> for ActionOptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionOptionPropertyIri || *other == ACTION_OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<ActionOptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionOptionPropertyIriOrLabel) -> bool {
		*self == ActionOptionPropertyIri || *self == ACTION_OPTION_PROPERTY_LABEL
	}
}
