/// <https://schema.org/Library>
pub const LIBRARY_IRI_HTTP: &str = "http://schema.org/Library";
/// <https://schema.org/Library>
pub const LIBRARY_IRI_HTTPS: &str = "https://schema.org/Library";
/// <https://schema.org/Library>
pub const LIBRARY_LABEL: &str = "Library";
pub struct LibraryIri;
impl PartialEq<&str> for LibraryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIBRARY_IRI_HTTP || *other == LIBRARY_IRI_HTTPS
	}
}
impl PartialEq<LibraryIri> for &str {
	fn eq(&self, other: &LibraryIri) -> bool {
		*self == LIBRARY_IRI_HTTP || *self == LIBRARY_IRI_HTTPS
	}
}
pub struct LibraryIriOrLabel;
impl PartialEq<&str> for LibraryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LibraryIri || *other == LIBRARY_LABEL
	}
}
impl PartialEq<LibraryIriOrLabel> for &str {
	fn eq(&self, other: &LibraryIriOrLabel) -> bool {
		*self == LibraryIri || *self == LIBRARY_LABEL
	}
}
