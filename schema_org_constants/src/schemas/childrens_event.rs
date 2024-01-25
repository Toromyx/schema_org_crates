/// <https://schema.org/ChildrensEvent>
pub const CHILDRENS_EVENT_IRI_HTTP: &str = "http://schema.org/ChildrensEvent";
/// <https://schema.org/ChildrensEvent>
pub const CHILDRENS_EVENT_IRI_HTTPS: &str = "https://schema.org/ChildrensEvent";
/// <https://schema.org/ChildrensEvent>
pub const CHILDRENS_EVENT_LABEL: &str = "ChildrensEvent";
pub struct ChildrensEventIri;
impl PartialEq<&str> for ChildrensEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILDRENS_EVENT_IRI_HTTP || *other == CHILDRENS_EVENT_IRI_HTTPS
	}
}
impl PartialEq<ChildrensEventIri> for &str {
	fn eq(&self, other: &ChildrensEventIri) -> bool {
		*self == CHILDRENS_EVENT_IRI_HTTP || *self == CHILDRENS_EVENT_IRI_HTTPS
	}
}
pub struct ChildrensEventIriOrLabel;
impl PartialEq<&str> for ChildrensEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildrensEventIri || *other == CHILDRENS_EVENT_LABEL
	}
}
impl PartialEq<ChildrensEventIriOrLabel> for &str {
	fn eq(&self, other: &ChildrensEventIriOrLabel) -> bool {
		*self == ChildrensEventIri || *self == CHILDRENS_EVENT_LABEL
	}
}
