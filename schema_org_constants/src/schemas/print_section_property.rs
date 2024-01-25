/// <https://schema.org/printSection>
pub const PRINT_SECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/printSection";
/// <https://schema.org/printSection>
pub const PRINT_SECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/printSection";
/// <https://schema.org/printSection>
pub const PRINT_SECTION_PROPERTY_LABEL: &str = "printSection";
pub struct PrintSectionPropertyIri;
impl PartialEq<&str> for PrintSectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRINT_SECTION_PROPERTY_IRI_HTTP || *other == PRINT_SECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrintSectionPropertyIri> for &str {
	fn eq(&self, other: &PrintSectionPropertyIri) -> bool {
		*self == PRINT_SECTION_PROPERTY_IRI_HTTP || *self == PRINT_SECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct PrintSectionPropertyIriOrLabel;
impl PartialEq<&str> for PrintSectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrintSectionPropertyIri || *other == PRINT_SECTION_PROPERTY_LABEL
	}
}
impl PartialEq<PrintSectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrintSectionPropertyIriOrLabel) -> bool {
		*self == PrintSectionPropertyIri || *self == PRINT_SECTION_PROPERTY_LABEL
	}
}
