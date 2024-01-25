/// <https://schema.org/InformAction>
pub const INFORM_ACTION_IRI_HTTP: &str = "http://schema.org/InformAction";
/// <https://schema.org/InformAction>
pub const INFORM_ACTION_IRI_HTTPS: &str = "https://schema.org/InformAction";
/// <https://schema.org/InformAction>
pub const INFORM_ACTION_LABEL: &str = "InformAction";
pub struct InformActionIri;
impl PartialEq<&str> for InformActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFORM_ACTION_IRI_HTTP || *other == INFORM_ACTION_IRI_HTTPS
	}
}
impl PartialEq<InformActionIri> for &str {
	fn eq(&self, other: &InformActionIri) -> bool {
		*self == INFORM_ACTION_IRI_HTTP || *self == INFORM_ACTION_IRI_HTTPS
	}
}
pub struct InformActionIriOrLabel;
impl PartialEq<&str> for InformActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InformActionIri || *other == INFORM_ACTION_LABEL
	}
}
impl PartialEq<InformActionIriOrLabel> for &str {
	fn eq(&self, other: &InformActionIriOrLabel) -> bool {
		*self == InformActionIri || *self == INFORM_ACTION_LABEL
	}
}
