/// <https://schema.org/vatID>
pub const VAT_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/vatID";
/// <https://schema.org/vatID>
pub const VAT_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vatID";
/// <https://schema.org/vatID>
pub const VAT_ID_PROPERTY_LABEL: &str = "vatID";
pub struct VatIdPropertyIri;
impl PartialEq<&str> for VatIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VAT_ID_PROPERTY_IRI_HTTP || *other == VAT_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VatIdPropertyIri> for &str {
	fn eq(&self, other: &VatIdPropertyIri) -> bool {
		*self == VAT_ID_PROPERTY_IRI_HTTP || *self == VAT_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct VatIdPropertyIriOrLabel;
impl PartialEq<&str> for VatIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VatIdPropertyIri || *other == VAT_ID_PROPERTY_LABEL
	}
}
impl PartialEq<VatIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VatIdPropertyIriOrLabel) -> bool {
		*self == VatIdPropertyIri || *self == VAT_ID_PROPERTY_LABEL
	}
}
