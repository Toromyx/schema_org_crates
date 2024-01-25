/// <https://schema.org/Bakery>
pub const BAKERY_IRI_HTTP: &str = "http://schema.org/Bakery";
/// <https://schema.org/Bakery>
pub const BAKERY_IRI_HTTPS: &str = "https://schema.org/Bakery";
/// <https://schema.org/Bakery>
pub const BAKERY_LABEL: &str = "Bakery";
pub struct BakeryIri;
impl PartialEq<&str> for BakeryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BAKERY_IRI_HTTP || *other == BAKERY_IRI_HTTPS
	}
}
impl PartialEq<BakeryIri> for &str {
	fn eq(&self, other: &BakeryIri) -> bool {
		*self == BAKERY_IRI_HTTP || *self == BAKERY_IRI_HTTPS
	}
}
pub struct BakeryIriOrLabel;
impl PartialEq<&str> for BakeryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BakeryIri || *other == BAKERY_LABEL
	}
}
impl PartialEq<BakeryIriOrLabel> for &str {
	fn eq(&self, other: &BakeryIriOrLabel) -> bool {
		*self == BakeryIri || *self == BAKERY_LABEL
	}
}
