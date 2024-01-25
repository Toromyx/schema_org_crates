/// <https://schema.org/printPage>
pub const PRINT_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/printPage";
/// <https://schema.org/printPage>
pub const PRINT_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/printPage";
/// <https://schema.org/printPage>
pub const PRINT_PAGE_PROPERTY_LABEL: &str = "printPage";
pub struct PrintPagePropertyIri;
impl PartialEq<&str> for PrintPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRINT_PAGE_PROPERTY_IRI_HTTP || *other == PRINT_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrintPagePropertyIri> for &str {
	fn eq(&self, other: &PrintPagePropertyIri) -> bool {
		*self == PRINT_PAGE_PROPERTY_IRI_HTTP || *self == PRINT_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct PrintPagePropertyIriOrLabel;
impl PartialEq<&str> for PrintPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrintPagePropertyIri || *other == PRINT_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<PrintPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrintPagePropertyIriOrLabel) -> bool {
		*self == PrintPagePropertyIri || *self == PRINT_PAGE_PROPERTY_LABEL
	}
}
