/// <https://schema.org/BedType>
pub const BED_TYPE_IRI_HTTP: &str = "http://schema.org/BedType";
/// <https://schema.org/BedType>
pub const BED_TYPE_IRI_HTTPS: &str = "https://schema.org/BedType";
/// <https://schema.org/BedType>
pub const BED_TYPE_LABEL: &str = "BedType";
pub struct BedTypeIri;
impl PartialEq<&str> for BedTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BED_TYPE_IRI_HTTP || *other == BED_TYPE_IRI_HTTPS
	}
}
impl PartialEq<BedTypeIri> for &str {
	fn eq(&self, other: &BedTypeIri) -> bool {
		*self == BED_TYPE_IRI_HTTP || *self == BED_TYPE_IRI_HTTPS
	}
}
pub struct BedTypeIriOrLabel;
impl PartialEq<&str> for BedTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BedTypeIri || *other == BED_TYPE_LABEL
	}
}
impl PartialEq<BedTypeIriOrLabel> for &str {
	fn eq(&self, other: &BedTypeIriOrLabel) -> bool {
		*self == BedTypeIri || *self == BED_TYPE_LABEL
	}
}
