/// <https://schema.org/EatAction>
pub const EAT_ACTION_IRI_HTTP: &str = "http://schema.org/EatAction";
/// <https://schema.org/EatAction>
pub const EAT_ACTION_IRI_HTTPS: &str = "https://schema.org/EatAction";
/// <https://schema.org/EatAction>
pub const EAT_ACTION_LABEL: &str = "EatAction";
pub struct EatActionIri;
impl PartialEq<&str> for EatActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EAT_ACTION_IRI_HTTP || *other == EAT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<EatActionIri> for &str {
	fn eq(&self, other: &EatActionIri) -> bool {
		*self == EAT_ACTION_IRI_HTTP || *self == EAT_ACTION_IRI_HTTPS
	}
}
pub struct EatActionIriOrLabel;
impl PartialEq<&str> for EatActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EatActionIri || *other == EAT_ACTION_LABEL
	}
}
impl PartialEq<EatActionIriOrLabel> for &str {
	fn eq(&self, other: &EatActionIriOrLabel) -> bool {
		*self == EatActionIri || *self == EAT_ACTION_LABEL
	}
}
