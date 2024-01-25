/// <https://schema.org/variantCover>
pub const VARIANT_COVER_PROPERTY_IRI_HTTP: &str = "http://schema.org/variantCover";
/// <https://schema.org/variantCover>
pub const VARIANT_COVER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/variantCover";
/// <https://schema.org/variantCover>
pub const VARIANT_COVER_PROPERTY_LABEL: &str = "variantCover";
pub struct VariantCoverPropertyIri;
impl PartialEq<&str> for VariantCoverPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VARIANT_COVER_PROPERTY_IRI_HTTP || *other == VARIANT_COVER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VariantCoverPropertyIri> for &str {
	fn eq(&self, other: &VariantCoverPropertyIri) -> bool {
		*self == VARIANT_COVER_PROPERTY_IRI_HTTP || *self == VARIANT_COVER_PROPERTY_IRI_HTTPS
	}
}
pub struct VariantCoverPropertyIriOrLabel;
impl PartialEq<&str> for VariantCoverPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VariantCoverPropertyIri || *other == VARIANT_COVER_PROPERTY_LABEL
	}
}
impl PartialEq<VariantCoverPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VariantCoverPropertyIriOrLabel) -> bool {
		*self == VariantCoverPropertyIri || *self == VARIANT_COVER_PROPERTY_LABEL
	}
}
