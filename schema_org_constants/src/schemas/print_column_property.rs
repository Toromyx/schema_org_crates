/// <https://schema.org/printColumn>
pub const PRINT_COLUMN_PROPERTY_IRI_HTTP: &str = "http://schema.org/printColumn";
/// <https://schema.org/printColumn>
pub const PRINT_COLUMN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/printColumn";
/// <https://schema.org/printColumn>
pub const PRINT_COLUMN_PROPERTY_LABEL: &str = "printColumn";
pub struct PrintColumnPropertyIri;
impl PartialEq<&str> for PrintColumnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRINT_COLUMN_PROPERTY_IRI_HTTP || *other == PRINT_COLUMN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrintColumnPropertyIri> for &str {
	fn eq(&self, other: &PrintColumnPropertyIri) -> bool {
		*self == PRINT_COLUMN_PROPERTY_IRI_HTTP || *self == PRINT_COLUMN_PROPERTY_IRI_HTTPS
	}
}
pub struct PrintColumnPropertyIriOrLabel;
impl PartialEq<&str> for PrintColumnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrintColumnPropertyIri || *other == PRINT_COLUMN_PROPERTY_LABEL
	}
}
impl PartialEq<PrintColumnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrintColumnPropertyIriOrLabel) -> bool {
		*self == PrintColumnPropertyIri || *self == PRINT_COLUMN_PROPERTY_LABEL
	}
}
