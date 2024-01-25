/// <https://schema.org/SpeakableSpecification>
pub const SPEAKABLE_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/SpeakableSpecification";
/// <https://schema.org/SpeakableSpecification>
pub const SPEAKABLE_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/SpeakableSpecification";
/// <https://schema.org/SpeakableSpecification>
pub const SPEAKABLE_SPECIFICATION_LABEL: &str = "SpeakableSpecification";
pub struct SpeakableSpecificationIri;
impl PartialEq<&str> for SpeakableSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPEAKABLE_SPECIFICATION_IRI_HTTP || *other == SPEAKABLE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<SpeakableSpecificationIri> for &str {
	fn eq(&self, other: &SpeakableSpecificationIri) -> bool {
		*self == SPEAKABLE_SPECIFICATION_IRI_HTTP || *self == SPEAKABLE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct SpeakableSpecificationIriOrLabel;
impl PartialEq<&str> for SpeakableSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpeakableSpecificationIri || *other == SPEAKABLE_SPECIFICATION_LABEL
	}
}
impl PartialEq<SpeakableSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &SpeakableSpecificationIriOrLabel) -> bool {
		*self == SpeakableSpecificationIri || *self == SPEAKABLE_SPECIFICATION_LABEL
	}
}
