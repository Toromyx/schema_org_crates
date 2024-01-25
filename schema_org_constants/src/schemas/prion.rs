/// <https://schema.org/Prion>
pub const PRION_IRI_HTTP: &str = "http://schema.org/Prion";
/// <https://schema.org/Prion>
pub const PRION_IRI_HTTPS: &str = "https://schema.org/Prion";
/// <https://schema.org/Prion>
pub const PRION_LABEL: &str = "Prion";
pub struct PrionIri;
impl PartialEq<&str> for PrionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRION_IRI_HTTP || *other == PRION_IRI_HTTPS
	}
}
impl PartialEq<PrionIri> for &str {
	fn eq(&self, other: &PrionIri) -> bool {
		*self == PRION_IRI_HTTP || *self == PRION_IRI_HTTPS
	}
}
pub struct PrionIriOrLabel;
impl PartialEq<&str> for PrionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrionIri || *other == PRION_LABEL
	}
}
impl PartialEq<PrionIriOrLabel> for &str {
	fn eq(&self, other: &PrionIriOrLabel) -> bool {
		*self == PrionIri || *self == PRION_LABEL
	}
}
