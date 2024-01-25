/// <https://schema.org/inChIKey>
pub const IN_CH_I_KEY_PROPERTY_IRI_HTTP: &str = "http://schema.org/inChIKey";
/// <https://schema.org/inChIKey>
pub const IN_CH_I_KEY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inChIKey";
/// <https://schema.org/inChIKey>
pub const IN_CH_I_KEY_PROPERTY_LABEL: &str = "inChIKey";
pub struct InChIKeyPropertyIri;
impl PartialEq<&str> for InChIKeyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_CH_I_KEY_PROPERTY_IRI_HTTP || *other == IN_CH_I_KEY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InChIKeyPropertyIri> for &str {
	fn eq(&self, other: &InChIKeyPropertyIri) -> bool {
		*self == IN_CH_I_KEY_PROPERTY_IRI_HTTP || *self == IN_CH_I_KEY_PROPERTY_IRI_HTTPS
	}
}
pub struct InChIKeyPropertyIriOrLabel;
impl PartialEq<&str> for InChIKeyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InChIKeyPropertyIri || *other == IN_CH_I_KEY_PROPERTY_LABEL
	}
}
impl PartialEq<InChIKeyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InChIKeyPropertyIriOrLabel) -> bool {
		*self == InChIKeyPropertyIri || *self == IN_CH_I_KEY_PROPERTY_LABEL
	}
}
