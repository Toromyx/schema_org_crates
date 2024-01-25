/// <https://schema.org/inCodeSet>
pub const IN_CODE_SET_PROPERTY_IRI_HTTP: &str = "http://schema.org/inCodeSet";
/// <https://schema.org/inCodeSet>
pub const IN_CODE_SET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inCodeSet";
/// <https://schema.org/inCodeSet>
pub const IN_CODE_SET_PROPERTY_LABEL: &str = "inCodeSet";
pub struct InCodeSetPropertyIri;
impl PartialEq<&str> for InCodeSetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_CODE_SET_PROPERTY_IRI_HTTP || *other == IN_CODE_SET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InCodeSetPropertyIri> for &str {
	fn eq(&self, other: &InCodeSetPropertyIri) -> bool {
		*self == IN_CODE_SET_PROPERTY_IRI_HTTP || *self == IN_CODE_SET_PROPERTY_IRI_HTTPS
	}
}
pub struct InCodeSetPropertyIriOrLabel;
impl PartialEq<&str> for InCodeSetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InCodeSetPropertyIri || *other == IN_CODE_SET_PROPERTY_LABEL
	}
}
impl PartialEq<InCodeSetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InCodeSetPropertyIriOrLabel) -> bool {
		*self == InCodeSetPropertyIri || *self == IN_CODE_SET_PROPERTY_LABEL
	}
}
