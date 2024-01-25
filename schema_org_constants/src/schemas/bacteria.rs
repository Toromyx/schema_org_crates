/// <https://schema.org/Bacteria>
pub const BACTERIA_IRI_HTTP: &str = "http://schema.org/Bacteria";
/// <https://schema.org/Bacteria>
pub const BACTERIA_IRI_HTTPS: &str = "https://schema.org/Bacteria";
/// <https://schema.org/Bacteria>
pub const BACTERIA_LABEL: &str = "Bacteria";
pub struct BacteriaIri;
impl PartialEq<&str> for BacteriaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BACTERIA_IRI_HTTP || *other == BACTERIA_IRI_HTTPS
	}
}
impl PartialEq<BacteriaIri> for &str {
	fn eq(&self, other: &BacteriaIri) -> bool {
		*self == BACTERIA_IRI_HTTP || *self == BACTERIA_IRI_HTTPS
	}
}
pub struct BacteriaIriOrLabel;
impl PartialEq<&str> for BacteriaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BacteriaIri || *other == BACTERIA_LABEL
	}
}
impl PartialEq<BacteriaIriOrLabel> for &str {
	fn eq(&self, other: &BacteriaIriOrLabel) -> bool {
		*self == BacteriaIri || *self == BACTERIA_LABEL
	}
}
