/// <https://schema.org/Pediatric>
pub const PEDIATRIC_IRI_HTTP: &str = "http://schema.org/Pediatric";
/// <https://schema.org/Pediatric>
pub const PEDIATRIC_IRI_HTTPS: &str = "https://schema.org/Pediatric";
/// <https://schema.org/Pediatric>
pub const PEDIATRIC_LABEL: &str = "Pediatric";
pub struct PediatricIri;
impl PartialEq<&str> for PediatricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PEDIATRIC_IRI_HTTP || *other == PEDIATRIC_IRI_HTTPS
	}
}
impl PartialEq<PediatricIri> for &str {
	fn eq(&self, other: &PediatricIri) -> bool {
		*self == PEDIATRIC_IRI_HTTP || *self == PEDIATRIC_IRI_HTTPS
	}
}
pub struct PediatricIriOrLabel;
impl PartialEq<&str> for PediatricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PediatricIri || *other == PEDIATRIC_LABEL
	}
}
impl PartialEq<PediatricIriOrLabel> for &str {
	fn eq(&self, other: &PediatricIriOrLabel) -> bool {
		*self == PediatricIri || *self == PEDIATRIC_LABEL
	}
}
