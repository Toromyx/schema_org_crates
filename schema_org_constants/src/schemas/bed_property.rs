/// <https://schema.org/bed>
pub const BED_PROPERTY_IRI_HTTP: &str = "http://schema.org/bed";
/// <https://schema.org/bed>
pub const BED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bed";
/// <https://schema.org/bed>
pub const BED_PROPERTY_LABEL: &str = "bed";
pub struct BedPropertyIri;
impl PartialEq<&str> for BedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BED_PROPERTY_IRI_HTTP || *other == BED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BedPropertyIri> for &str {
	fn eq(&self, other: &BedPropertyIri) -> bool {
		*self == BED_PROPERTY_IRI_HTTP || *self == BED_PROPERTY_IRI_HTTPS
	}
}
pub struct BedPropertyIriOrLabel;
impl PartialEq<&str> for BedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BedPropertyIri || *other == BED_PROPERTY_LABEL
	}
}
impl PartialEq<BedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BedPropertyIriOrLabel) -> bool {
		*self == BedPropertyIri || *self == BED_PROPERTY_LABEL
	}
}
