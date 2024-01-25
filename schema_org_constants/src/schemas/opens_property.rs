/// <https://schema.org/opens>
pub const OPENS_PROPERTY_IRI_HTTP: &str = "http://schema.org/opens";
/// <https://schema.org/opens>
pub const OPENS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/opens";
/// <https://schema.org/opens>
pub const OPENS_PROPERTY_LABEL: &str = "opens";
pub struct OpensPropertyIri;
impl PartialEq<&str> for OpensPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPENS_PROPERTY_IRI_HTTP || *other == OPENS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OpensPropertyIri> for &str {
	fn eq(&self, other: &OpensPropertyIri) -> bool {
		*self == OPENS_PROPERTY_IRI_HTTP || *self == OPENS_PROPERTY_IRI_HTTPS
	}
}
pub struct OpensPropertyIriOrLabel;
impl PartialEq<&str> for OpensPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpensPropertyIri || *other == OPENS_PROPERTY_LABEL
	}
}
impl PartialEq<OpensPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OpensPropertyIriOrLabel) -> bool {
		*self == OpensPropertyIri || *self == OPENS_PROPERTY_LABEL
	}
}
