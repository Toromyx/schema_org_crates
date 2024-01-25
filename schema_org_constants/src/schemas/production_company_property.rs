/// <https://schema.org/productionCompany>
pub const PRODUCTION_COMPANY_PROPERTY_IRI_HTTP: &str = "http://schema.org/productionCompany";
/// <https://schema.org/productionCompany>
pub const PRODUCTION_COMPANY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productionCompany";
/// <https://schema.org/productionCompany>
pub const PRODUCTION_COMPANY_PROPERTY_LABEL: &str = "productionCompany";
pub struct ProductionCompanyPropertyIri;
impl PartialEq<&str> for ProductionCompanyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCTION_COMPANY_PROPERTY_IRI_HTTP
			|| *other == PRODUCTION_COMPANY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductionCompanyPropertyIri> for &str {
	fn eq(&self, other: &ProductionCompanyPropertyIri) -> bool {
		*self == PRODUCTION_COMPANY_PROPERTY_IRI_HTTP
			|| *self == PRODUCTION_COMPANY_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductionCompanyPropertyIriOrLabel;
impl PartialEq<&str> for ProductionCompanyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductionCompanyPropertyIri || *other == PRODUCTION_COMPANY_PROPERTY_LABEL
	}
}
impl PartialEq<ProductionCompanyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductionCompanyPropertyIriOrLabel) -> bool {
		*self == ProductionCompanyPropertyIri || *self == PRODUCTION_COMPANY_PROPERTY_LABEL
	}
}
