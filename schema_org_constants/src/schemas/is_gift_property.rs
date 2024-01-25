/// <https://schema.org/isGift>
pub const IS_GIFT_PROPERTY_IRI_HTTP: &str = "http://schema.org/isGift";
/// <https://schema.org/isGift>
pub const IS_GIFT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isGift";
/// <https://schema.org/isGift>
pub const IS_GIFT_PROPERTY_LABEL: &str = "isGift";
pub struct IsGiftPropertyIri;
impl PartialEq<&str> for IsGiftPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_GIFT_PROPERTY_IRI_HTTP || *other == IS_GIFT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsGiftPropertyIri> for &str {
	fn eq(&self, other: &IsGiftPropertyIri) -> bool {
		*self == IS_GIFT_PROPERTY_IRI_HTTP || *self == IS_GIFT_PROPERTY_IRI_HTTPS
	}
}
pub struct IsGiftPropertyIriOrLabel;
impl PartialEq<&str> for IsGiftPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsGiftPropertyIri || *other == IS_GIFT_PROPERTY_LABEL
	}
}
impl PartialEq<IsGiftPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsGiftPropertyIriOrLabel) -> bool {
		*self == IsGiftPropertyIri || *self == IS_GIFT_PROPERTY_LABEL
	}
}
