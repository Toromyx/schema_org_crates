/// <https://schema.org/taxID>
pub const TAX_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/taxID";
/// <https://schema.org/taxID>
pub const TAX_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/taxID";
/// <https://schema.org/taxID>
pub const TAX_ID_PROPERTY_LABEL: &str = "taxID";
pub struct TaxIdPropertyIri;
impl PartialEq<&str> for TaxIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAX_ID_PROPERTY_IRI_HTTP || *other == TAX_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TaxIdPropertyIri> for &str {
	fn eq(&self, other: &TaxIdPropertyIri) -> bool {
		*self == TAX_ID_PROPERTY_IRI_HTTP || *self == TAX_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct TaxIdPropertyIriOrLabel;
impl PartialEq<&str> for TaxIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxIdPropertyIri || *other == TAX_ID_PROPERTY_LABEL
	}
}
impl PartialEq<TaxIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TaxIdPropertyIriOrLabel) -> bool {
		*self == TaxIdPropertyIri || *self == TAX_ID_PROPERTY_LABEL
	}
}
