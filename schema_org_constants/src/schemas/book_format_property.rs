/// <https://schema.org/bookFormat>
pub const BOOK_FORMAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/bookFormat";
/// <https://schema.org/bookFormat>
pub const BOOK_FORMAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bookFormat";
/// <https://schema.org/bookFormat>
pub const BOOK_FORMAT_PROPERTY_LABEL: &str = "bookFormat";
pub struct BookFormatPropertyIri;
impl PartialEq<&str> for BookFormatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_FORMAT_PROPERTY_IRI_HTTP || *other == BOOK_FORMAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BookFormatPropertyIri> for &str {
	fn eq(&self, other: &BookFormatPropertyIri) -> bool {
		*self == BOOK_FORMAT_PROPERTY_IRI_HTTP || *self == BOOK_FORMAT_PROPERTY_IRI_HTTPS
	}
}
pub struct BookFormatPropertyIriOrLabel;
impl PartialEq<&str> for BookFormatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookFormatPropertyIri || *other == BOOK_FORMAT_PROPERTY_LABEL
	}
}
impl PartialEq<BookFormatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BookFormatPropertyIriOrLabel) -> bool {
		*self == BookFormatPropertyIri || *self == BOOK_FORMAT_PROPERTY_LABEL
	}
}
