/// <https://schema.org/borrower>
pub const BORROWER_PROPERTY_IRI_HTTP: &str = "http://schema.org/borrower";
/// <https://schema.org/borrower>
pub const BORROWER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/borrower";
/// <https://schema.org/borrower>
pub const BORROWER_PROPERTY_LABEL: &str = "borrower";
pub struct BorrowerPropertyIri;
impl PartialEq<&str> for BorrowerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BORROWER_PROPERTY_IRI_HTTP || *other == BORROWER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BorrowerPropertyIri> for &str {
	fn eq(&self, other: &BorrowerPropertyIri) -> bool {
		*self == BORROWER_PROPERTY_IRI_HTTP || *self == BORROWER_PROPERTY_IRI_HTTPS
	}
}
pub struct BorrowerPropertyIriOrLabel;
impl PartialEq<&str> for BorrowerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BorrowerPropertyIri || *other == BORROWER_PROPERTY_LABEL
	}
}
impl PartialEq<BorrowerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BorrowerPropertyIriOrLabel) -> bool {
		*self == BorrowerPropertyIri || *self == BORROWER_PROPERTY_LABEL
	}
}
