/// <https://schema.org/Corporation>
pub const CORPORATION_IRI_HTTP: &str = "http://schema.org/Corporation";
/// <https://schema.org/Corporation>
pub const CORPORATION_IRI_HTTPS: &str = "https://schema.org/Corporation";
/// <https://schema.org/Corporation>
pub const CORPORATION_LABEL: &str = "Corporation";
pub struct CorporationIri;
impl PartialEq<&str> for CorporationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CORPORATION_IRI_HTTP || *other == CORPORATION_IRI_HTTPS
	}
}
impl PartialEq<CorporationIri> for &str {
	fn eq(&self, other: &CorporationIri) -> bool {
		*self == CORPORATION_IRI_HTTP || *self == CORPORATION_IRI_HTTPS
	}
}
pub struct CorporationIriOrLabel;
impl PartialEq<&str> for CorporationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CorporationIri || *other == CORPORATION_LABEL
	}
}
impl PartialEq<CorporationIriOrLabel> for &str {
	fn eq(&self, other: &CorporationIriOrLabel) -> bool {
		*self == CorporationIri || *self == CORPORATION_LABEL
	}
}
