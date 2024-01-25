/// <https://schema.org/preOp>
pub const PRE_OP_PROPERTY_IRI_HTTP: &str = "http://schema.org/preOp";
/// <https://schema.org/preOp>
pub const PRE_OP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/preOp";
/// <https://schema.org/preOp>
pub const PRE_OP_PROPERTY_LABEL: &str = "preOp";
pub struct PreOpPropertyIri;
impl PartialEq<&str> for PreOpPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRE_OP_PROPERTY_IRI_HTTP || *other == PRE_OP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PreOpPropertyIri> for &str {
	fn eq(&self, other: &PreOpPropertyIri) -> bool {
		*self == PRE_OP_PROPERTY_IRI_HTTP || *self == PRE_OP_PROPERTY_IRI_HTTPS
	}
}
pub struct PreOpPropertyIriOrLabel;
impl PartialEq<&str> for PreOpPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreOpPropertyIri || *other == PRE_OP_PROPERTY_LABEL
	}
}
impl PartialEq<PreOpPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PreOpPropertyIriOrLabel) -> bool {
		*self == PreOpPropertyIri || *self == PRE_OP_PROPERTY_LABEL
	}
}
