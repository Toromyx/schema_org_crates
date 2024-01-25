/// <https://schema.org/potentialUse>
pub const POTENTIAL_USE_PROPERTY_IRI_HTTP: &str = "http://schema.org/potentialUse";
/// <https://schema.org/potentialUse>
pub const POTENTIAL_USE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/potentialUse";
/// <https://schema.org/potentialUse>
pub const POTENTIAL_USE_PROPERTY_LABEL: &str = "potentialUse";
pub struct PotentialUsePropertyIri;
impl PartialEq<&str> for PotentialUsePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POTENTIAL_USE_PROPERTY_IRI_HTTP || *other == POTENTIAL_USE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PotentialUsePropertyIri> for &str {
	fn eq(&self, other: &PotentialUsePropertyIri) -> bool {
		*self == POTENTIAL_USE_PROPERTY_IRI_HTTP || *self == POTENTIAL_USE_PROPERTY_IRI_HTTPS
	}
}
pub struct PotentialUsePropertyIriOrLabel;
impl PartialEq<&str> for PotentialUsePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PotentialUsePropertyIri || *other == POTENTIAL_USE_PROPERTY_LABEL
	}
}
impl PartialEq<PotentialUsePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PotentialUsePropertyIriOrLabel) -> bool {
		*self == PotentialUsePropertyIri || *self == POTENTIAL_USE_PROPERTY_LABEL
	}
}
