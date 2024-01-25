/// <https://schema.org/hasDefinedTerm>
pub const HAS_DEFINED_TERM_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasDefinedTerm";
/// <https://schema.org/hasDefinedTerm>
pub const HAS_DEFINED_TERM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasDefinedTerm";
/// <https://schema.org/hasDefinedTerm>
pub const HAS_DEFINED_TERM_PROPERTY_LABEL: &str = "hasDefinedTerm";
pub struct HasDefinedTermPropertyIri;
impl PartialEq<&str> for HasDefinedTermPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_DEFINED_TERM_PROPERTY_IRI_HTTP
			|| *other == HAS_DEFINED_TERM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasDefinedTermPropertyIri> for &str {
	fn eq(&self, other: &HasDefinedTermPropertyIri) -> bool {
		*self == HAS_DEFINED_TERM_PROPERTY_IRI_HTTP || *self == HAS_DEFINED_TERM_PROPERTY_IRI_HTTPS
	}
}
pub struct HasDefinedTermPropertyIriOrLabel;
impl PartialEq<&str> for HasDefinedTermPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasDefinedTermPropertyIri || *other == HAS_DEFINED_TERM_PROPERTY_LABEL
	}
}
impl PartialEq<HasDefinedTermPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasDefinedTermPropertyIriOrLabel) -> bool {
		*self == HasDefinedTermPropertyIri || *self == HAS_DEFINED_TERM_PROPERTY_LABEL
	}
}
