/// <https://schema.org/DaySpa>
pub const DAY_SPA_IRI_HTTP: &str = "http://schema.org/DaySpa";
/// <https://schema.org/DaySpa>
pub const DAY_SPA_IRI_HTTPS: &str = "https://schema.org/DaySpa";
/// <https://schema.org/DaySpa>
pub const DAY_SPA_LABEL: &str = "DaySpa";
pub struct DaySpaIri;
impl PartialEq<&str> for DaySpaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DAY_SPA_IRI_HTTP || *other == DAY_SPA_IRI_HTTPS
	}
}
impl PartialEq<DaySpaIri> for &str {
	fn eq(&self, other: &DaySpaIri) -> bool {
		*self == DAY_SPA_IRI_HTTP || *self == DAY_SPA_IRI_HTTPS
	}
}
pub struct DaySpaIriOrLabel;
impl PartialEq<&str> for DaySpaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DaySpaIri || *other == DAY_SPA_LABEL
	}
}
impl PartialEq<DaySpaIriOrLabel> for &str {
	fn eq(&self, other: &DaySpaIriOrLabel) -> bool {
		*self == DaySpaIri || *self == DAY_SPA_LABEL
	}
}
