/// <https://schema.org/productionDate>
pub const PRODUCTION_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/productionDate";
/// <https://schema.org/productionDate>
pub const PRODUCTION_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productionDate";
/// <https://schema.org/productionDate>
pub const PRODUCTION_DATE_PROPERTY_LABEL: &str = "productionDate";
pub struct ProductionDatePropertyIri;
impl PartialEq<&str> for ProductionDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCTION_DATE_PROPERTY_IRI_HTTP || *other == PRODUCTION_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductionDatePropertyIri> for &str {
	fn eq(&self, other: &ProductionDatePropertyIri) -> bool {
		*self == PRODUCTION_DATE_PROPERTY_IRI_HTTP || *self == PRODUCTION_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductionDatePropertyIriOrLabel;
impl PartialEq<&str> for ProductionDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductionDatePropertyIri || *other == PRODUCTION_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ProductionDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductionDatePropertyIriOrLabel) -> bool {
		*self == ProductionDatePropertyIri || *self == PRODUCTION_DATE_PROPERTY_LABEL
	}
}
