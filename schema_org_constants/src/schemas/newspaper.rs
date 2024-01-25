/// <https://schema.org/Newspaper>
pub const NEWSPAPER_IRI_HTTP: &str = "http://schema.org/Newspaper";
/// <https://schema.org/Newspaper>
pub const NEWSPAPER_IRI_HTTPS: &str = "https://schema.org/Newspaper";
/// <https://schema.org/Newspaper>
pub const NEWSPAPER_LABEL: &str = "Newspaper";
pub struct NewspaperIri;
impl PartialEq<&str> for NewspaperIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEWSPAPER_IRI_HTTP || *other == NEWSPAPER_IRI_HTTPS
	}
}
impl PartialEq<NewspaperIri> for &str {
	fn eq(&self, other: &NewspaperIri) -> bool {
		*self == NEWSPAPER_IRI_HTTP || *self == NEWSPAPER_IRI_HTTPS
	}
}
pub struct NewspaperIriOrLabel;
impl PartialEq<&str> for NewspaperIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NewspaperIri || *other == NEWSPAPER_LABEL
	}
}
impl PartialEq<NewspaperIriOrLabel> for &str {
	fn eq(&self, other: &NewspaperIriOrLabel) -> bool {
		*self == NewspaperIri || *self == NEWSPAPER_LABEL
	}
}
