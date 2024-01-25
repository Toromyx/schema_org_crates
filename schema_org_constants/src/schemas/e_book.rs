/// <https://schema.org/EBook>
pub const E_BOOK_IRI_HTTP: &str = "http://schema.org/EBook";
/// <https://schema.org/EBook>
pub const E_BOOK_IRI_HTTPS: &str = "https://schema.org/EBook";
/// <https://schema.org/EBook>
pub const E_BOOK_LABEL: &str = "EBook";
pub struct EBookIri;
impl PartialEq<&str> for EBookIri {
	fn eq(&self, other: &&str) -> bool {
		*other == E_BOOK_IRI_HTTP || *other == E_BOOK_IRI_HTTPS
	}
}
impl PartialEq<EBookIri> for &str {
	fn eq(&self, other: &EBookIri) -> bool {
		*self == E_BOOK_IRI_HTTP || *self == E_BOOK_IRI_HTTPS
	}
}
pub struct EBookIriOrLabel;
impl PartialEq<&str> for EBookIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EBookIri || *other == E_BOOK_LABEL
	}
}
impl PartialEq<EBookIriOrLabel> for &str {
	fn eq(&self, other: &EBookIriOrLabel) -> bool {
		*self == EBookIri || *self == E_BOOK_LABEL
	}
}
