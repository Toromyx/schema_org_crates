/// <https://schema.org/nerve>
pub const NERVE_PROPERTY_IRI_HTTP: &str = "http://schema.org/nerve";
/// <https://schema.org/nerve>
pub const NERVE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nerve";
/// <https://schema.org/nerve>
pub const NERVE_PROPERTY_LABEL: &str = "nerve";
pub struct NervePropertyIri;
impl PartialEq<&str> for NervePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NERVE_PROPERTY_IRI_HTTP || *other == NERVE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NervePropertyIri> for &str {
	fn eq(&self, other: &NervePropertyIri) -> bool {
		*self == NERVE_PROPERTY_IRI_HTTP || *self == NERVE_PROPERTY_IRI_HTTPS
	}
}
pub struct NervePropertyIriOrLabel;
impl PartialEq<&str> for NervePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NervePropertyIri || *other == NERVE_PROPERTY_LABEL
	}
}
impl PartialEq<NervePropertyIriOrLabel> for &str {
	fn eq(&self, other: &NervePropertyIriOrLabel) -> bool {
		*self == NervePropertyIri || *self == NERVE_PROPERTY_LABEL
	}
}
