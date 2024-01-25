/// <https://schema.org/CDFormat>
pub const CD_FORMAT_IRI_HTTP: &str = "http://schema.org/CDFormat";
/// <https://schema.org/CDFormat>
pub const CD_FORMAT_IRI_HTTPS: &str = "https://schema.org/CDFormat";
/// <https://schema.org/CDFormat>
pub const CD_FORMAT_LABEL: &str = "CDFormat";
pub struct CdFormatIri;
impl PartialEq<&str> for CdFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CD_FORMAT_IRI_HTTP || *other == CD_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<CdFormatIri> for &str {
	fn eq(&self, other: &CdFormatIri) -> bool {
		*self == CD_FORMAT_IRI_HTTP || *self == CD_FORMAT_IRI_HTTPS
	}
}
pub struct CdFormatIriOrLabel;
impl PartialEq<&str> for CdFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CdFormatIri || *other == CD_FORMAT_LABEL
	}
}
impl PartialEq<CdFormatIriOrLabel> for &str {
	fn eq(&self, other: &CdFormatIriOrLabel) -> bool {
		*self == CdFormatIri || *self == CD_FORMAT_LABEL
	}
}
