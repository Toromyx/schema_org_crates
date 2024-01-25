/// <https://schema.org/printEdition>
pub const PRINT_EDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/printEdition";
/// <https://schema.org/printEdition>
pub const PRINT_EDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/printEdition";
/// <https://schema.org/printEdition>
pub const PRINT_EDITION_PROPERTY_LABEL: &str = "printEdition";
pub struct PrintEditionPropertyIri;
impl PartialEq<&str> for PrintEditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRINT_EDITION_PROPERTY_IRI_HTTP || *other == PRINT_EDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrintEditionPropertyIri> for &str {
	fn eq(&self, other: &PrintEditionPropertyIri) -> bool {
		*self == PRINT_EDITION_PROPERTY_IRI_HTTP || *self == PRINT_EDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct PrintEditionPropertyIriOrLabel;
impl PartialEq<&str> for PrintEditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrintEditionPropertyIri || *other == PRINT_EDITION_PROPERTY_LABEL
	}
}
impl PartialEq<PrintEditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrintEditionPropertyIriOrLabel) -> bool {
		*self == PrintEditionPropertyIri || *self == PRINT_EDITION_PROPERTY_LABEL
	}
}
