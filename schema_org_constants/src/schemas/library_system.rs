/// <https://schema.org/LibrarySystem>
pub const LIBRARY_SYSTEM_IRI_HTTP: &str = "http://schema.org/LibrarySystem";
/// <https://schema.org/LibrarySystem>
pub const LIBRARY_SYSTEM_IRI_HTTPS: &str = "https://schema.org/LibrarySystem";
/// <https://schema.org/LibrarySystem>
pub const LIBRARY_SYSTEM_LABEL: &str = "LibrarySystem";
pub struct LibrarySystemIri;
impl PartialEq<&str> for LibrarySystemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIBRARY_SYSTEM_IRI_HTTP || *other == LIBRARY_SYSTEM_IRI_HTTPS
	}
}
impl PartialEq<LibrarySystemIri> for &str {
	fn eq(&self, other: &LibrarySystemIri) -> bool {
		*self == LIBRARY_SYSTEM_IRI_HTTP || *self == LIBRARY_SYSTEM_IRI_HTTPS
	}
}
pub struct LibrarySystemIriOrLabel;
impl PartialEq<&str> for LibrarySystemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LibrarySystemIri || *other == LIBRARY_SYSTEM_LABEL
	}
}
impl PartialEq<LibrarySystemIriOrLabel> for &str {
	fn eq(&self, other: &LibrarySystemIriOrLabel) -> bool {
		*self == LibrarySystemIri || *self == LIBRARY_SYSTEM_LABEL
	}
}
