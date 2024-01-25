/// <https://schema.org/subEvent>
pub const SUB_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/subEvent";
/// <https://schema.org/subEvent>
pub const SUB_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subEvent";
/// <https://schema.org/subEvent>
pub const SUB_EVENT_PROPERTY_LABEL: &str = "subEvent";
pub struct SubEventPropertyIri;
impl PartialEq<&str> for SubEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_EVENT_PROPERTY_IRI_HTTP || *other == SUB_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubEventPropertyIri> for &str {
	fn eq(&self, other: &SubEventPropertyIri) -> bool {
		*self == SUB_EVENT_PROPERTY_IRI_HTTP || *self == SUB_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SubEventPropertyIriOrLabel;
impl PartialEq<&str> for SubEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubEventPropertyIri || *other == SUB_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<SubEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubEventPropertyIriOrLabel) -> bool {
		*self == SubEventPropertyIri || *self == SUB_EVENT_PROPERTY_LABEL
	}
}
