/// <https://schema.org/Genitourinary>
pub const GENITOURINARY_IRI_HTTP: &str = "http://schema.org/Genitourinary";
/// <https://schema.org/Genitourinary>
pub const GENITOURINARY_IRI_HTTPS: &str = "https://schema.org/Genitourinary";
/// <https://schema.org/Genitourinary>
pub const GENITOURINARY_LABEL: &str = "Genitourinary";
pub struct GenitourinaryIri;
impl PartialEq<&str> for GenitourinaryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENITOURINARY_IRI_HTTP || *other == GENITOURINARY_IRI_HTTPS
	}
}
impl PartialEq<GenitourinaryIri> for &str {
	fn eq(&self, other: &GenitourinaryIri) -> bool {
		*self == GENITOURINARY_IRI_HTTP || *self == GENITOURINARY_IRI_HTTPS
	}
}
pub struct GenitourinaryIriOrLabel;
impl PartialEq<&str> for GenitourinaryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GenitourinaryIri || *other == GENITOURINARY_LABEL
	}
}
impl PartialEq<GenitourinaryIriOrLabel> for &str {
	fn eq(&self, other: &GenitourinaryIriOrLabel) -> bool {
		*self == GenitourinaryIri || *self == GENITOURINARY_LABEL
	}
}
