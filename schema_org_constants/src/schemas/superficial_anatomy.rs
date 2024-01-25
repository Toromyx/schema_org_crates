/// <https://schema.org/SuperficialAnatomy>
pub const SUPERFICIAL_ANATOMY_IRI_HTTP: &str = "http://schema.org/SuperficialAnatomy";
/// <https://schema.org/SuperficialAnatomy>
pub const SUPERFICIAL_ANATOMY_IRI_HTTPS: &str = "https://schema.org/SuperficialAnatomy";
/// <https://schema.org/SuperficialAnatomy>
pub const SUPERFICIAL_ANATOMY_LABEL: &str = "SuperficialAnatomy";
pub struct SuperficialAnatomyIri;
impl PartialEq<&str> for SuperficialAnatomyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPERFICIAL_ANATOMY_IRI_HTTP || *other == SUPERFICIAL_ANATOMY_IRI_HTTPS
	}
}
impl PartialEq<SuperficialAnatomyIri> for &str {
	fn eq(&self, other: &SuperficialAnatomyIri) -> bool {
		*self == SUPERFICIAL_ANATOMY_IRI_HTTP || *self == SUPERFICIAL_ANATOMY_IRI_HTTPS
	}
}
pub struct SuperficialAnatomyIriOrLabel;
impl PartialEq<&str> for SuperficialAnatomyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuperficialAnatomyIri || *other == SUPERFICIAL_ANATOMY_LABEL
	}
}
impl PartialEq<SuperficialAnatomyIriOrLabel> for &str {
	fn eq(&self, other: &SuperficialAnatomyIriOrLabel) -> bool {
		*self == SuperficialAnatomyIri || *self == SUPERFICIAL_ANATOMY_LABEL
	}
}
