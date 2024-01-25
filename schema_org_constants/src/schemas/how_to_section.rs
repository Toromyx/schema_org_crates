/// <https://schema.org/HowToSection>
pub const HOW_TO_SECTION_IRI_HTTP: &str = "http://schema.org/HowToSection";
/// <https://schema.org/HowToSection>
pub const HOW_TO_SECTION_IRI_HTTPS: &str = "https://schema.org/HowToSection";
/// <https://schema.org/HowToSection>
pub const HOW_TO_SECTION_LABEL: &str = "HowToSection";
pub struct HowToSectionIri;
impl PartialEq<&str> for HowToSectionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_SECTION_IRI_HTTP || *other == HOW_TO_SECTION_IRI_HTTPS
	}
}
impl PartialEq<HowToSectionIri> for &str {
	fn eq(&self, other: &HowToSectionIri) -> bool {
		*self == HOW_TO_SECTION_IRI_HTTP || *self == HOW_TO_SECTION_IRI_HTTPS
	}
}
pub struct HowToSectionIriOrLabel;
impl PartialEq<&str> for HowToSectionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToSectionIri || *other == HOW_TO_SECTION_LABEL
	}
}
impl PartialEq<HowToSectionIriOrLabel> for &str {
	fn eq(&self, other: &HowToSectionIriOrLabel) -> bool {
		*self == HowToSectionIri || *self == HOW_TO_SECTION_LABEL
	}
}
