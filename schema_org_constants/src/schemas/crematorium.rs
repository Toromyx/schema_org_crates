/// <https://schema.org/Crematorium>
pub const CREMATORIUM_IRI_HTTP: &str = "http://schema.org/Crematorium";
/// <https://schema.org/Crematorium>
pub const CREMATORIUM_IRI_HTTPS: &str = "https://schema.org/Crematorium";
/// <https://schema.org/Crematorium>
pub const CREMATORIUM_LABEL: &str = "Crematorium";
pub struct CrematoriumIri;
impl PartialEq<&str> for CrematoriumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREMATORIUM_IRI_HTTP || *other == CREMATORIUM_IRI_HTTPS
	}
}
impl PartialEq<CrematoriumIri> for &str {
	fn eq(&self, other: &CrematoriumIri) -> bool {
		*self == CREMATORIUM_IRI_HTTP || *self == CREMATORIUM_IRI_HTTPS
	}
}
pub struct CrematoriumIriOrLabel;
impl PartialEq<&str> for CrematoriumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CrematoriumIri || *other == CREMATORIUM_LABEL
	}
}
impl PartialEq<CrematoriumIriOrLabel> for &str {
	fn eq(&self, other: &CrematoriumIriOrLabel) -> bool {
		*self == CrematoriumIri || *self == CREMATORIUM_LABEL
	}
}
