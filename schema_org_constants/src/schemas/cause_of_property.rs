/// <https://schema.org/causeOf>
pub const CAUSE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/causeOf";
/// <https://schema.org/causeOf>
pub const CAUSE_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/causeOf";
/// <https://schema.org/causeOf>
pub const CAUSE_OF_PROPERTY_LABEL: &str = "causeOf";
pub struct CauseOfPropertyIri;
impl PartialEq<&str> for CauseOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAUSE_OF_PROPERTY_IRI_HTTP || *other == CAUSE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CauseOfPropertyIri> for &str {
	fn eq(&self, other: &CauseOfPropertyIri) -> bool {
		*self == CAUSE_OF_PROPERTY_IRI_HTTP || *self == CAUSE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct CauseOfPropertyIriOrLabel;
impl PartialEq<&str> for CauseOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CauseOfPropertyIri || *other == CAUSE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<CauseOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CauseOfPropertyIriOrLabel) -> bool {
		*self == CauseOfPropertyIri || *self == CAUSE_OF_PROPERTY_LABEL
	}
}
