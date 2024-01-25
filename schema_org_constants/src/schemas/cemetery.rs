/// <https://schema.org/Cemetery>
pub const CEMETERY_IRI_HTTP: &str = "http://schema.org/Cemetery";
/// <https://schema.org/Cemetery>
pub const CEMETERY_IRI_HTTPS: &str = "https://schema.org/Cemetery";
/// <https://schema.org/Cemetery>
pub const CEMETERY_LABEL: &str = "Cemetery";
pub struct CemeteryIri;
impl PartialEq<&str> for CemeteryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CEMETERY_IRI_HTTP || *other == CEMETERY_IRI_HTTPS
	}
}
impl PartialEq<CemeteryIri> for &str {
	fn eq(&self, other: &CemeteryIri) -> bool {
		*self == CEMETERY_IRI_HTTP || *self == CEMETERY_IRI_HTTPS
	}
}
pub struct CemeteryIriOrLabel;
impl PartialEq<&str> for CemeteryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CemeteryIri || *other == CEMETERY_LABEL
	}
}
impl PartialEq<CemeteryIriOrLabel> for &str {
	fn eq(&self, other: &CemeteryIriOrLabel) -> bool {
		*self == CemeteryIri || *self == CEMETERY_LABEL
	}
}
