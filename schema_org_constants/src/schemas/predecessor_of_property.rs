/// <https://schema.org/predecessorOf>
pub const PREDECESSOR_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/predecessorOf";
/// <https://schema.org/predecessorOf>
pub const PREDECESSOR_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/predecessorOf";
/// <https://schema.org/predecessorOf>
pub const PREDECESSOR_OF_PROPERTY_LABEL: &str = "predecessorOf";
pub struct PredecessorOfPropertyIri;
impl PartialEq<&str> for PredecessorOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREDECESSOR_OF_PROPERTY_IRI_HTTP || *other == PREDECESSOR_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PredecessorOfPropertyIri> for &str {
	fn eq(&self, other: &PredecessorOfPropertyIri) -> bool {
		*self == PREDECESSOR_OF_PROPERTY_IRI_HTTP || *self == PREDECESSOR_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct PredecessorOfPropertyIriOrLabel;
impl PartialEq<&str> for PredecessorOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PredecessorOfPropertyIri || *other == PREDECESSOR_OF_PROPERTY_LABEL
	}
}
impl PartialEq<PredecessorOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PredecessorOfPropertyIriOrLabel) -> bool {
		*self == PredecessorOfPropertyIri || *self == PREDECESSOR_OF_PROPERTY_LABEL
	}
}
