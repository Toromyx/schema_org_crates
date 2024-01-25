/// <https://schema.org/hasVariant>
pub const HAS_VARIANT_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasVariant";
/// <https://schema.org/hasVariant>
pub const HAS_VARIANT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasVariant";
/// <https://schema.org/hasVariant>
pub const HAS_VARIANT_PROPERTY_LABEL: &str = "hasVariant";
pub struct HasVariantPropertyIri;
impl PartialEq<&str> for HasVariantPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_VARIANT_PROPERTY_IRI_HTTP || *other == HAS_VARIANT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasVariantPropertyIri> for &str {
	fn eq(&self, other: &HasVariantPropertyIri) -> bool {
		*self == HAS_VARIANT_PROPERTY_IRI_HTTP || *self == HAS_VARIANT_PROPERTY_IRI_HTTPS
	}
}
pub struct HasVariantPropertyIriOrLabel;
impl PartialEq<&str> for HasVariantPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasVariantPropertyIri || *other == HAS_VARIANT_PROPERTY_LABEL
	}
}
impl PartialEq<HasVariantPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasVariantPropertyIriOrLabel) -> bool {
		*self == HasVariantPropertyIri || *self == HAS_VARIANT_PROPERTY_LABEL
	}
}
