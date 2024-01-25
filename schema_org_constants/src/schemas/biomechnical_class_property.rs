/// <https://schema.org/biomechnicalClass>
pub const BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTP: &str = "http://schema.org/biomechnicalClass";
/// <https://schema.org/biomechnicalClass>
pub const BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/biomechnicalClass";
/// <https://schema.org/biomechnicalClass>
pub const BIOMECHNICAL_CLASS_PROPERTY_LABEL: &str = "biomechnicalClass";
pub struct BiomechnicalClassPropertyIri;
impl PartialEq<&str> for BiomechnicalClassPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTP
			|| *other == BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BiomechnicalClassPropertyIri> for &str {
	fn eq(&self, other: &BiomechnicalClassPropertyIri) -> bool {
		*self == BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTP
			|| *self == BIOMECHNICAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
pub struct BiomechnicalClassPropertyIriOrLabel;
impl PartialEq<&str> for BiomechnicalClassPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BiomechnicalClassPropertyIri || *other == BIOMECHNICAL_CLASS_PROPERTY_LABEL
	}
}
impl PartialEq<BiomechnicalClassPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BiomechnicalClassPropertyIriOrLabel) -> bool {
		*self == BiomechnicalClassPropertyIri || *self == BIOMECHNICAL_CLASS_PROPERTY_LABEL
	}
}
