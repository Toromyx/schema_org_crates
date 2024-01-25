/// <https://schema.org/Throat>
pub const THROAT_IRI_HTTP: &str = "http://schema.org/Throat";
/// <https://schema.org/Throat>
pub const THROAT_IRI_HTTPS: &str = "https://schema.org/Throat";
/// <https://schema.org/Throat>
pub const THROAT_LABEL: &str = "Throat";
pub struct ThroatIri;
impl PartialEq<&str> for ThroatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THROAT_IRI_HTTP || *other == THROAT_IRI_HTTPS
	}
}
impl PartialEq<ThroatIri> for &str {
	fn eq(&self, other: &ThroatIri) -> bool {
		*self == THROAT_IRI_HTTP || *self == THROAT_IRI_HTTPS
	}
}
pub struct ThroatIriOrLabel;
impl PartialEq<&str> for ThroatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThroatIri || *other == THROAT_LABEL
	}
}
impl PartialEq<ThroatIriOrLabel> for &str {
	fn eq(&self, other: &ThroatIriOrLabel) -> bool {
		*self == ThroatIri || *self == THROAT_LABEL
	}
}
