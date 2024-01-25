/// <https://schema.org/BedDetails>
pub const BED_DETAILS_IRI_HTTP: &str = "http://schema.org/BedDetails";
/// <https://schema.org/BedDetails>
pub const BED_DETAILS_IRI_HTTPS: &str = "https://schema.org/BedDetails";
/// <https://schema.org/BedDetails>
pub const BED_DETAILS_LABEL: &str = "BedDetails";
pub struct BedDetailsIri;
impl PartialEq<&str> for BedDetailsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BED_DETAILS_IRI_HTTP || *other == BED_DETAILS_IRI_HTTPS
	}
}
impl PartialEq<BedDetailsIri> for &str {
	fn eq(&self, other: &BedDetailsIri) -> bool {
		*self == BED_DETAILS_IRI_HTTP || *self == BED_DETAILS_IRI_HTTPS
	}
}
pub struct BedDetailsIriOrLabel;
impl PartialEq<&str> for BedDetailsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BedDetailsIri || *other == BED_DETAILS_LABEL
	}
}
impl PartialEq<BedDetailsIriOrLabel> for &str {
	fn eq(&self, other: &BedDetailsIriOrLabel) -> bool {
		*self == BedDetailsIri || *self == BED_DETAILS_LABEL
	}
}
