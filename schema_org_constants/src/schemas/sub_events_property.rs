/// <https://schema.org/subEvents>
#[deprecated = "This schema is superseded by <https://schema.org/subEvent>."]
pub const SUB_EVENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/subEvents";
/// <https://schema.org/subEvents>
#[deprecated = "This schema is superseded by <https://schema.org/subEvent>."]
pub const SUB_EVENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subEvents";
/// <https://schema.org/subEvents>
#[deprecated = "This schema is superseded by <https://schema.org/subEvent>."]
pub const SUB_EVENTS_PROPERTY_LABEL: &str = "subEvents";
pub struct SubEventsPropertyIri;
impl PartialEq<&str> for SubEventsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_EVENTS_PROPERTY_IRI_HTTP || *other == SUB_EVENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubEventsPropertyIri> for &str {
	fn eq(&self, other: &SubEventsPropertyIri) -> bool {
		*self == SUB_EVENTS_PROPERTY_IRI_HTTP || *self == SUB_EVENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct SubEventsPropertyIriOrLabel;
impl PartialEq<&str> for SubEventsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubEventsPropertyIri || *other == SUB_EVENTS_PROPERTY_LABEL
	}
}
impl PartialEq<SubEventsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubEventsPropertyIriOrLabel) -> bool {
		*self == SubEventsPropertyIri || *self == SUB_EVENTS_PROPERTY_LABEL
	}
}
