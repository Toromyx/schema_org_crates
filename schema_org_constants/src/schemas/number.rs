/// <https://schema.org/Number>
pub const NUMBER_IRI_HTTP: &str = "http://schema.org/Number";
/// <https://schema.org/Number>
pub const NUMBER_IRI_HTTPS: &str = "https://schema.org/Number";
/// <https://schema.org/Number>
pub const NUMBER_LABEL: &str = "Number";
pub struct NumberIri;
impl PartialEq<&str> for NumberIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_IRI_HTTP || *other == NUMBER_IRI_HTTPS
	}
}
impl PartialEq<NumberIri> for &str {
	fn eq(&self, other: &NumberIri) -> bool {
		*self == NUMBER_IRI_HTTP || *self == NUMBER_IRI_HTTPS
	}
}
pub struct NumberIriOrLabel;
impl PartialEq<&str> for NumberIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberIri || *other == NUMBER_LABEL
	}
}
impl PartialEq<NumberIriOrLabel> for &str {
	fn eq(&self, other: &NumberIriOrLabel) -> bool {
		*self == NumberIri || *self == NUMBER_LABEL
	}
}
