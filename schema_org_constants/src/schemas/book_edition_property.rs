/// <https://schema.org/bookEdition>
pub const BOOK_EDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/bookEdition";
/// <https://schema.org/bookEdition>
pub const BOOK_EDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bookEdition";
/// <https://schema.org/bookEdition>
pub const BOOK_EDITION_PROPERTY_LABEL: &str = "bookEdition";
pub struct BookEditionPropertyIri;
impl PartialEq<&str> for BookEditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_EDITION_PROPERTY_IRI_HTTP || *other == BOOK_EDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BookEditionPropertyIri> for &str {
	fn eq(&self, other: &BookEditionPropertyIri) -> bool {
		*self == BOOK_EDITION_PROPERTY_IRI_HTTP || *self == BOOK_EDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct BookEditionPropertyIriOrLabel;
impl PartialEq<&str> for BookEditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookEditionPropertyIri || *other == BOOK_EDITION_PROPERTY_LABEL
	}
}
impl PartialEq<BookEditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BookEditionPropertyIriOrLabel) -> bool {
		*self == BookEditionPropertyIri || *self == BOOK_EDITION_PROPERTY_LABEL
	}
}
