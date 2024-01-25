/// <https://schema.org/BusinessEvent>
pub const BUSINESS_EVENT_IRI_HTTP: &str = "http://schema.org/BusinessEvent";
/// <https://schema.org/BusinessEvent>
pub const BUSINESS_EVENT_IRI_HTTPS: &str = "https://schema.org/BusinessEvent";
/// <https://schema.org/BusinessEvent>
pub const BUSINESS_EVENT_LABEL: &str = "BusinessEvent";
pub struct BusinessEventIri;
impl PartialEq<&str> for BusinessEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_EVENT_IRI_HTTP || *other == BUSINESS_EVENT_IRI_HTTPS
	}
}
impl PartialEq<BusinessEventIri> for &str {
	fn eq(&self, other: &BusinessEventIri) -> bool {
		*self == BUSINESS_EVENT_IRI_HTTP || *self == BUSINESS_EVENT_IRI_HTTPS
	}
}
pub struct BusinessEventIriOrLabel;
impl PartialEq<&str> for BusinessEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessEventIri || *other == BUSINESS_EVENT_LABEL
	}
}
impl PartialEq<BusinessEventIriOrLabel> for &str {
	fn eq(&self, other: &BusinessEventIriOrLabel) -> bool {
		*self == BusinessEventIri || *self == BUSINESS_EVENT_LABEL
	}
}
