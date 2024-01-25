/// <https://schema.org/ChooseAction>
pub const CHOOSE_ACTION_IRI_HTTP: &str = "http://schema.org/ChooseAction";
/// <https://schema.org/ChooseAction>
pub const CHOOSE_ACTION_IRI_HTTPS: &str = "https://schema.org/ChooseAction";
/// <https://schema.org/ChooseAction>
pub const CHOOSE_ACTION_LABEL: &str = "ChooseAction";
pub struct ChooseActionIri;
impl PartialEq<&str> for ChooseActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHOOSE_ACTION_IRI_HTTP || *other == CHOOSE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ChooseActionIri> for &str {
	fn eq(&self, other: &ChooseActionIri) -> bool {
		*self == CHOOSE_ACTION_IRI_HTTP || *self == CHOOSE_ACTION_IRI_HTTPS
	}
}
pub struct ChooseActionIriOrLabel;
impl PartialEq<&str> for ChooseActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChooseActionIri || *other == CHOOSE_ACTION_LABEL
	}
}
impl PartialEq<ChooseActionIriOrLabel> for &str {
	fn eq(&self, other: &ChooseActionIriOrLabel) -> bool {
		*self == ChooseActionIri || *self == CHOOSE_ACTION_LABEL
	}
}
