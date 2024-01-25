/// <https://schema.org/successorOf>
pub const SUCCESSOR_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/successorOf";
/// <https://schema.org/successorOf>
pub const SUCCESSOR_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/successorOf";
/// <https://schema.org/successorOf>
pub const SUCCESSOR_OF_PROPERTY_LABEL: &str = "successorOf";
pub struct SuccessorOfPropertyIri;
impl PartialEq<&str> for SuccessorOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUCCESSOR_OF_PROPERTY_IRI_HTTP || *other == SUCCESSOR_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuccessorOfPropertyIri> for &str {
	fn eq(&self, other: &SuccessorOfPropertyIri) -> bool {
		*self == SUCCESSOR_OF_PROPERTY_IRI_HTTP || *self == SUCCESSOR_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct SuccessorOfPropertyIriOrLabel;
impl PartialEq<&str> for SuccessorOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuccessorOfPropertyIri || *other == SUCCESSOR_OF_PROPERTY_LABEL
	}
}
impl PartialEq<SuccessorOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuccessorOfPropertyIriOrLabel) -> bool {
		*self == SuccessorOfPropertyIri || *self == SUCCESSOR_OF_PROPERTY_LABEL
	}
}
