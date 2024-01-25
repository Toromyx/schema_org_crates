/// <https://schema.org/inDefinedTermSet>
pub const IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTP: &str = "http://schema.org/inDefinedTermSet";
/// <https://schema.org/inDefinedTermSet>
pub const IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inDefinedTermSet";
/// <https://schema.org/inDefinedTermSet>
pub const IN_DEFINED_TERM_SET_PROPERTY_LABEL: &str = "inDefinedTermSet";
pub struct InDefinedTermSetPropertyIri;
impl PartialEq<&str> for InDefinedTermSetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTP
			|| *other == IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InDefinedTermSetPropertyIri> for &str {
	fn eq(&self, other: &InDefinedTermSetPropertyIri) -> bool {
		*self == IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTP
			|| *self == IN_DEFINED_TERM_SET_PROPERTY_IRI_HTTPS
	}
}
pub struct InDefinedTermSetPropertyIriOrLabel;
impl PartialEq<&str> for InDefinedTermSetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InDefinedTermSetPropertyIri || *other == IN_DEFINED_TERM_SET_PROPERTY_LABEL
	}
}
impl PartialEq<InDefinedTermSetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InDefinedTermSetPropertyIriOrLabel) -> bool {
		*self == InDefinedTermSetPropertyIri || *self == IN_DEFINED_TERM_SET_PROPERTY_LABEL
	}
}
