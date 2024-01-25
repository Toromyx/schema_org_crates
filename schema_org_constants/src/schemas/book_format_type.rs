/// <https://schema.org/BookFormatType>
pub const BOOK_FORMAT_TYPE_IRI_HTTP: &str = "http://schema.org/BookFormatType";
/// <https://schema.org/BookFormatType>
pub const BOOK_FORMAT_TYPE_IRI_HTTPS: &str = "https://schema.org/BookFormatType";
/// <https://schema.org/BookFormatType>
pub const BOOK_FORMAT_TYPE_LABEL: &str = "BookFormatType";
pub struct BookFormatTypeIri;
impl PartialEq<&str> for BookFormatTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_FORMAT_TYPE_IRI_HTTP || *other == BOOK_FORMAT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<BookFormatTypeIri> for &str {
	fn eq(&self, other: &BookFormatTypeIri) -> bool {
		*self == BOOK_FORMAT_TYPE_IRI_HTTP || *self == BOOK_FORMAT_TYPE_IRI_HTTPS
	}
}
pub struct BookFormatTypeIriOrLabel;
impl PartialEq<&str> for BookFormatTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookFormatTypeIri || *other == BOOK_FORMAT_TYPE_LABEL
	}
}
impl PartialEq<BookFormatTypeIriOrLabel> for &str {
	fn eq(&self, other: &BookFormatTypeIriOrLabel) -> bool {
		*self == BookFormatTypeIri || *self == BOOK_FORMAT_TYPE_LABEL
	}
}
