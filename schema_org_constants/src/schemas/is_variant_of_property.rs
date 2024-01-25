/// <https://schema.org/isVariantOf>
pub const IS_VARIANT_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/isVariantOf";
/// <https://schema.org/isVariantOf>
pub const IS_VARIANT_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isVariantOf";
/// <https://schema.org/isVariantOf>
pub const IS_VARIANT_OF_PROPERTY_LABEL: &str = "isVariantOf";
pub struct IsVariantOfPropertyIri;
impl PartialEq<&str> for IsVariantOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_VARIANT_OF_PROPERTY_IRI_HTTP || *other == IS_VARIANT_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsVariantOfPropertyIri> for &str {
	fn eq(&self, other: &IsVariantOfPropertyIri) -> bool {
		*self == IS_VARIANT_OF_PROPERTY_IRI_HTTP || *self == IS_VARIANT_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct IsVariantOfPropertyIriOrLabel;
impl PartialEq<&str> for IsVariantOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsVariantOfPropertyIri || *other == IS_VARIANT_OF_PROPERTY_LABEL
	}
}
impl PartialEq<IsVariantOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsVariantOfPropertyIriOrLabel) -> bool {
		*self == IsVariantOfPropertyIri || *self == IS_VARIANT_OF_PROPERTY_LABEL
	}
}
