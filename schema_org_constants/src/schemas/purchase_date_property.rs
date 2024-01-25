/// <https://schema.org/purchaseDate>
pub const PURCHASE_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/purchaseDate";
/// <https://schema.org/purchaseDate>
pub const PURCHASE_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/purchaseDate";
/// <https://schema.org/purchaseDate>
pub const PURCHASE_DATE_PROPERTY_LABEL: &str = "purchaseDate";
pub struct PurchaseDatePropertyIri;
impl PartialEq<&str> for PurchaseDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PURCHASE_DATE_PROPERTY_IRI_HTTP || *other == PURCHASE_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PurchaseDatePropertyIri> for &str {
	fn eq(&self, other: &PurchaseDatePropertyIri) -> bool {
		*self == PURCHASE_DATE_PROPERTY_IRI_HTTP || *self == PURCHASE_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct PurchaseDatePropertyIriOrLabel;
impl PartialEq<&str> for PurchaseDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PurchaseDatePropertyIri || *other == PURCHASE_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<PurchaseDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PurchaseDatePropertyIriOrLabel) -> bool {
		*self == PurchaseDatePropertyIri || *self == PURCHASE_DATE_PROPERTY_LABEL
	}
}
