/// <https://schema.org/releasedEvent>
pub const RELEASED_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/releasedEvent";
/// <https://schema.org/releasedEvent>
pub const RELEASED_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/releasedEvent";
/// <https://schema.org/releasedEvent>
pub const RELEASED_EVENT_PROPERTY_LABEL: &str = "releasedEvent";
pub struct ReleasedEventPropertyIri;
impl PartialEq<&str> for ReleasedEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEASED_EVENT_PROPERTY_IRI_HTTP || *other == RELEASED_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReleasedEventPropertyIri> for &str {
	fn eq(&self, other: &ReleasedEventPropertyIri) -> bool {
		*self == RELEASED_EVENT_PROPERTY_IRI_HTTP || *self == RELEASED_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ReleasedEventPropertyIriOrLabel;
impl PartialEq<&str> for ReleasedEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReleasedEventPropertyIri || *other == RELEASED_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<ReleasedEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReleasedEventPropertyIriOrLabel) -> bool {
		*self == ReleasedEventPropertyIri || *self == RELEASED_EVENT_PROPERTY_LABEL
	}
}
