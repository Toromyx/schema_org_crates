/// <https://schema.org/Nerve>
pub const NERVE_IRI_HTTP: &str = "http://schema.org/Nerve";
/// <https://schema.org/Nerve>
pub const NERVE_IRI_HTTPS: &str = "https://schema.org/Nerve";
/// <https://schema.org/Nerve>
pub const NERVE_LABEL: &str = "Nerve";
pub struct NerveIri;
impl PartialEq<&str> for NerveIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NERVE_IRI_HTTP || *other == NERVE_IRI_HTTPS
	}
}
impl PartialEq<NerveIri> for &str {
	fn eq(&self, other: &NerveIri) -> bool {
		*self == NERVE_IRI_HTTP || *self == NERVE_IRI_HTTPS
	}
}
pub struct NerveIriOrLabel;
impl PartialEq<&str> for NerveIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NerveIri || *other == NERVE_LABEL
	}
}
impl PartialEq<NerveIriOrLabel> for &str {
	fn eq(&self, other: &NerveIriOrLabel) -> bool {
		*self == NerveIri || *self == NERVE_LABEL
	}
}
