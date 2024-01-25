/// <https://schema.org/ComedyEvent>
pub const COMEDY_EVENT_IRI_HTTP: &str = "http://schema.org/ComedyEvent";
/// <https://schema.org/ComedyEvent>
pub const COMEDY_EVENT_IRI_HTTPS: &str = "https://schema.org/ComedyEvent";
/// <https://schema.org/ComedyEvent>
pub const COMEDY_EVENT_LABEL: &str = "ComedyEvent";
pub struct ComedyEventIri;
impl PartialEq<&str> for ComedyEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMEDY_EVENT_IRI_HTTP || *other == COMEDY_EVENT_IRI_HTTPS
	}
}
impl PartialEq<ComedyEventIri> for &str {
	fn eq(&self, other: &ComedyEventIri) -> bool {
		*self == COMEDY_EVENT_IRI_HTTP || *self == COMEDY_EVENT_IRI_HTTPS
	}
}
pub struct ComedyEventIriOrLabel;
impl PartialEq<&str> for ComedyEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComedyEventIri || *other == COMEDY_EVENT_LABEL
	}
}
impl PartialEq<ComedyEventIriOrLabel> for &str {
	fn eq(&self, other: &ComedyEventIriOrLabel) -> bool {
		*self == ComedyEventIri || *self == COMEDY_EVENT_LABEL
	}
}
