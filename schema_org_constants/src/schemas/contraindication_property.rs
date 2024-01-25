/// <https://schema.org/contraindication>
pub const CONTRAINDICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/contraindication";
/// <https://schema.org/contraindication>
pub const CONTRAINDICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contraindication";
/// <https://schema.org/contraindication>
pub const CONTRAINDICATION_PROPERTY_LABEL: &str = "contraindication";
pub struct ContraindicationPropertyIri;
impl PartialEq<&str> for ContraindicationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTRAINDICATION_PROPERTY_IRI_HTTP
			|| *other == CONTRAINDICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContraindicationPropertyIri> for &str {
	fn eq(&self, other: &ContraindicationPropertyIri) -> bool {
		*self == CONTRAINDICATION_PROPERTY_IRI_HTTP || *self == CONTRAINDICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ContraindicationPropertyIriOrLabel;
impl PartialEq<&str> for ContraindicationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContraindicationPropertyIri || *other == CONTRAINDICATION_PROPERTY_LABEL
	}
}
impl PartialEq<ContraindicationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContraindicationPropertyIriOrLabel) -> bool {
		*self == ContraindicationPropertyIri || *self == CONTRAINDICATION_PROPERTY_LABEL
	}
}
