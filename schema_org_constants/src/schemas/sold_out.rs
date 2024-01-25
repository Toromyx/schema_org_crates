/// <https://schema.org/SoldOut>
pub const SOLD_OUT_IRI_HTTP: &str = "http://schema.org/SoldOut";
/// <https://schema.org/SoldOut>
pub const SOLD_OUT_IRI_HTTPS: &str = "https://schema.org/SoldOut";
/// <https://schema.org/SoldOut>
pub const SOLD_OUT_LABEL: &str = "SoldOut";
pub struct SoldOutIri;
impl PartialEq<&str> for SoldOutIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOLD_OUT_IRI_HTTP || *other == SOLD_OUT_IRI_HTTPS
	}
}
impl PartialEq<SoldOutIri> for &str {
	fn eq(&self, other: &SoldOutIri) -> bool {
		*self == SOLD_OUT_IRI_HTTP || *self == SOLD_OUT_IRI_HTTPS
	}
}
pub struct SoldOutIriOrLabel;
impl PartialEq<&str> for SoldOutIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoldOutIri || *other == SOLD_OUT_LABEL
	}
}
impl PartialEq<SoldOutIriOrLabel> for &str {
	fn eq(&self, other: &SoldOutIriOrLabel) -> bool {
		*self == SoldOutIri || *self == SOLD_OUT_LABEL
	}
}
