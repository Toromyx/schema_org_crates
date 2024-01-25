/// <https://schema.org/domiciledMortgage>
pub const DOMICILED_MORTGAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/domiciledMortgage";
/// <https://schema.org/domiciledMortgage>
pub const DOMICILED_MORTGAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/domiciledMortgage";
/// <https://schema.org/domiciledMortgage>
pub const DOMICILED_MORTGAGE_PROPERTY_LABEL: &str = "domiciledMortgage";
pub struct DomiciledMortgagePropertyIri;
impl PartialEq<&str> for DomiciledMortgagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOMICILED_MORTGAGE_PROPERTY_IRI_HTTP
			|| *other == DOMICILED_MORTGAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DomiciledMortgagePropertyIri> for &str {
	fn eq(&self, other: &DomiciledMortgagePropertyIri) -> bool {
		*self == DOMICILED_MORTGAGE_PROPERTY_IRI_HTTP
			|| *self == DOMICILED_MORTGAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct DomiciledMortgagePropertyIriOrLabel;
impl PartialEq<&str> for DomiciledMortgagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DomiciledMortgagePropertyIri || *other == DOMICILED_MORTGAGE_PROPERTY_LABEL
	}
}
impl PartialEq<DomiciledMortgagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DomiciledMortgagePropertyIriOrLabel) -> bool {
		*self == DomiciledMortgagePropertyIri || *self == DOMICILED_MORTGAGE_PROPERTY_LABEL
	}
}
