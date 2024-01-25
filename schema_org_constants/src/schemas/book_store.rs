/// <https://schema.org/BookStore>
pub const BOOK_STORE_IRI_HTTP: &str = "http://schema.org/BookStore";
/// <https://schema.org/BookStore>
pub const BOOK_STORE_IRI_HTTPS: &str = "https://schema.org/BookStore";
/// <https://schema.org/BookStore>
pub const BOOK_STORE_LABEL: &str = "BookStore";
pub struct BookStoreIri;
impl PartialEq<&str> for BookStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOK_STORE_IRI_HTTP || *other == BOOK_STORE_IRI_HTTPS
	}
}
impl PartialEq<BookStoreIri> for &str {
	fn eq(&self, other: &BookStoreIri) -> bool {
		*self == BOOK_STORE_IRI_HTTP || *self == BOOK_STORE_IRI_HTTPS
	}
}
pub struct BookStoreIriOrLabel;
impl PartialEq<&str> for BookStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookStoreIri || *other == BOOK_STORE_LABEL
	}
}
impl PartialEq<BookStoreIriOrLabel> for &str {
	fn eq(&self, other: &BookStoreIriOrLabel) -> bool {
		*self == BookStoreIri || *self == BOOK_STORE_LABEL
	}
}
