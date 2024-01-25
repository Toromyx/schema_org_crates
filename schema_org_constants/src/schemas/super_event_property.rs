/// <https://schema.org/superEvent>
pub const SUPER_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/superEvent";
/// <https://schema.org/superEvent>
pub const SUPER_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/superEvent";
/// <https://schema.org/superEvent>
pub const SUPER_EVENT_PROPERTY_LABEL: &str = "superEvent";
pub struct SuperEventPropertyIri;
impl PartialEq<&str> for SuperEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPER_EVENT_PROPERTY_IRI_HTTP || *other == SUPER_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuperEventPropertyIri> for &str {
	fn eq(&self, other: &SuperEventPropertyIri) -> bool {
		*self == SUPER_EVENT_PROPERTY_IRI_HTTP || *self == SUPER_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SuperEventPropertyIriOrLabel;
impl PartialEq<&str> for SuperEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuperEventPropertyIri || *other == SUPER_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<SuperEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuperEventPropertyIriOrLabel) -> bool {
		*self == SuperEventPropertyIri || *self == SUPER_EVENT_PROPERTY_LABEL
	}
}
