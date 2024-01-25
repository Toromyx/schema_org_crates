/// <https://schema.org/numberOfPages>
pub const NUMBER_OF_PAGES_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfPages";
/// <https://schema.org/numberOfPages>
pub const NUMBER_OF_PAGES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfPages";
/// <https://schema.org/numberOfPages>
pub const NUMBER_OF_PAGES_PROPERTY_LABEL: &str = "numberOfPages";
pub struct NumberOfPagesPropertyIri;
impl PartialEq<&str> for NumberOfPagesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_PAGES_PROPERTY_IRI_HTTP || *other == NUMBER_OF_PAGES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfPagesPropertyIri> for &str {
	fn eq(&self, other: &NumberOfPagesPropertyIri) -> bool {
		*self == NUMBER_OF_PAGES_PROPERTY_IRI_HTTP || *self == NUMBER_OF_PAGES_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfPagesPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfPagesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfPagesPropertyIri || *other == NUMBER_OF_PAGES_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfPagesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfPagesPropertyIriOrLabel) -> bool {
		*self == NumberOfPagesPropertyIri || *self == NUMBER_OF_PAGES_PROPERTY_LABEL
	}
}
