/// <https://schema.org/IndividualProduct>
pub const INDIVIDUAL_PRODUCT_IRI_HTTP: &str = "http://schema.org/IndividualProduct";
/// <https://schema.org/IndividualProduct>
pub const INDIVIDUAL_PRODUCT_IRI_HTTPS: &str = "https://schema.org/IndividualProduct";
/// <https://schema.org/IndividualProduct>
pub const INDIVIDUAL_PRODUCT_LABEL: &str = "IndividualProduct";
pub struct IndividualProductIri;
impl PartialEq<&str> for IndividualProductIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INDIVIDUAL_PRODUCT_IRI_HTTP || *other == INDIVIDUAL_PRODUCT_IRI_HTTPS
	}
}
impl PartialEq<IndividualProductIri> for &str {
	fn eq(&self, other: &IndividualProductIri) -> bool {
		*self == INDIVIDUAL_PRODUCT_IRI_HTTP || *self == INDIVIDUAL_PRODUCT_IRI_HTTPS
	}
}
pub struct IndividualProductIriOrLabel;
impl PartialEq<&str> for IndividualProductIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IndividualProductIri || *other == INDIVIDUAL_PRODUCT_LABEL
	}
}
impl PartialEq<IndividualProductIriOrLabel> for &str {
	fn eq(&self, other: &IndividualProductIriOrLabel) -> bool {
		*self == IndividualProductIri || *self == INDIVIDUAL_PRODUCT_LABEL
	}
}
