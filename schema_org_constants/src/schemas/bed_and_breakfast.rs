/// <https://schema.org/BedAndBreakfast>
pub const BED_AND_BREAKFAST_IRI_HTTP: &str = "http://schema.org/BedAndBreakfast";
/// <https://schema.org/BedAndBreakfast>
pub const BED_AND_BREAKFAST_IRI_HTTPS: &str = "https://schema.org/BedAndBreakfast";
/// <https://schema.org/BedAndBreakfast>
pub const BED_AND_BREAKFAST_LABEL: &str = "BedAndBreakfast";
pub struct BedAndBreakfastIri;
impl PartialEq<&str> for BedAndBreakfastIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BED_AND_BREAKFAST_IRI_HTTP || *other == BED_AND_BREAKFAST_IRI_HTTPS
	}
}
impl PartialEq<BedAndBreakfastIri> for &str {
	fn eq(&self, other: &BedAndBreakfastIri) -> bool {
		*self == BED_AND_BREAKFAST_IRI_HTTP || *self == BED_AND_BREAKFAST_IRI_HTTPS
	}
}
pub struct BedAndBreakfastIriOrLabel;
impl PartialEq<&str> for BedAndBreakfastIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BedAndBreakfastIri || *other == BED_AND_BREAKFAST_LABEL
	}
}
impl PartialEq<BedAndBreakfastIriOrLabel> for &str {
	fn eq(&self, other: &BedAndBreakfastIriOrLabel) -> bool {
		*self == BedAndBreakfastIri || *self == BED_AND_BREAKFAST_LABEL
	}
}
