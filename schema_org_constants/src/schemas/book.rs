/// <https://schema.org/Book>
pub const BOOK_IRI_HTTP: &str = "http://schema.org/Book";
/// <https://schema.org/Book>
pub const BOOK_IRI_HTTPS: &str = "https://schema.org/Book";
/// <https://schema.org/Book>
pub const BOOK_LABEL: &str = "Book";
pub struct BookIri;
impl PartialEq<&str> for BookIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_IRI_HTTP || *other == BOOK_IRI_HTTPS
	}
}
impl PartialEq<BookIri> for &str {
	fn eq(&self, other: &BookIri) -> bool {
		*self == BOOK_IRI_HTTP || *self == BOOK_IRI_HTTPS
	}
}
pub struct BookIriOrLabel;
impl PartialEq<&str> for BookIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookIri || *other == BOOK_LABEL
	}
}
impl PartialEq<BookIriOrLabel> for &str {
	fn eq(&self, other: &BookIriOrLabel) -> bool {
		*self == BookIri || *self == BOOK_LABEL
	}
}
