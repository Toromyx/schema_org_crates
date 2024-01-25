/// <https://schema.org/mechanismOfAction>
pub const MECHANISM_OF_ACTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/mechanismOfAction";
/// <https://schema.org/mechanismOfAction>
pub const MECHANISM_OF_ACTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mechanismOfAction";
/// <https://schema.org/mechanismOfAction>
pub const MECHANISM_OF_ACTION_PROPERTY_LABEL: &str = "mechanismOfAction";
pub struct MechanismOfActionPropertyIri;
impl PartialEq<&str> for MechanismOfActionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MECHANISM_OF_ACTION_PROPERTY_IRI_HTTP
			|| *other == MECHANISM_OF_ACTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MechanismOfActionPropertyIri> for &str {
	fn eq(&self, other: &MechanismOfActionPropertyIri) -> bool {
		*self == MECHANISM_OF_ACTION_PROPERTY_IRI_HTTP
			|| *self == MECHANISM_OF_ACTION_PROPERTY_IRI_HTTPS
	}
}
pub struct MechanismOfActionPropertyIriOrLabel;
impl PartialEq<&str> for MechanismOfActionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MechanismOfActionPropertyIri || *other == MECHANISM_OF_ACTION_PROPERTY_LABEL
	}
}
impl PartialEq<MechanismOfActionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MechanismOfActionPropertyIriOrLabel) -> bool {
		*self == MechanismOfActionPropertyIri || *self == MECHANISM_OF_ACTION_PROPERTY_LABEL
	}
}
