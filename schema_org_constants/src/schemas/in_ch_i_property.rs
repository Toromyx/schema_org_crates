/// <https://schema.org/inChI>
pub const IN_CH_I_PROPERTY_IRI_HTTP: &str = "http://schema.org/inChI";
/// <https://schema.org/inChI>
pub const IN_CH_I_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inChI";
/// <https://schema.org/inChI>
pub const IN_CH_I_PROPERTY_LABEL: &str = "inChI";
pub struct InChIPropertyIri;
impl PartialEq<&str> for InChIPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_CH_I_PROPERTY_IRI_HTTP || *other == IN_CH_I_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InChIPropertyIri> for &str {
	fn eq(&self, other: &InChIPropertyIri) -> bool {
		*self == IN_CH_I_PROPERTY_IRI_HTTP || *self == IN_CH_I_PROPERTY_IRI_HTTPS
	}
}
pub struct InChIPropertyIriOrLabel;
impl PartialEq<&str> for InChIPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InChIPropertyIri || *other == IN_CH_I_PROPERTY_LABEL
	}
}
impl PartialEq<InChIPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InChIPropertyIriOrLabel) -> bool {
		*self == InChIPropertyIri || *self == IN_CH_I_PROPERTY_LABEL
	}
}
