/// <https://schema.org/PrintDigitalSource>
pub const PRINT_DIGITAL_SOURCE_IRI_HTTP: &str = "http://schema.org/PrintDigitalSource";
/// <https://schema.org/PrintDigitalSource>
pub const PRINT_DIGITAL_SOURCE_IRI_HTTPS: &str = "https://schema.org/PrintDigitalSource";
/// <https://schema.org/PrintDigitalSource>
pub const PRINT_DIGITAL_SOURCE_LABEL: &str = "PrintDigitalSource";
pub struct PrintDigitalSourceIri;
impl PartialEq<&str> for PrintDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRINT_DIGITAL_SOURCE_IRI_HTTP || *other == PRINT_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<PrintDigitalSourceIri> for &str {
	fn eq(&self, other: &PrintDigitalSourceIri) -> bool {
		*self == PRINT_DIGITAL_SOURCE_IRI_HTTP || *self == PRINT_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct PrintDigitalSourceIriOrLabel;
impl PartialEq<&str> for PrintDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrintDigitalSourceIri || *other == PRINT_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<PrintDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &PrintDigitalSourceIriOrLabel) -> bool {
		*self == PrintDigitalSourceIri || *self == PRINT_DIGITAL_SOURCE_LABEL
	}
}
