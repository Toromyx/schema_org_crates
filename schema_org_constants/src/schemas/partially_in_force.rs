/// <https://schema.org/PartiallyInForce>
pub const PARTIALLY_IN_FORCE_IRI_HTTP: &str = "http://schema.org/PartiallyInForce";
/// <https://schema.org/PartiallyInForce>
pub const PARTIALLY_IN_FORCE_IRI_HTTPS: &str = "https://schema.org/PartiallyInForce";
/// <https://schema.org/PartiallyInForce>
pub const PARTIALLY_IN_FORCE_LABEL: &str = "PartiallyInForce";
pub struct PartiallyInForceIri;
impl PartialEq<&str> for PartiallyInForceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARTIALLY_IN_FORCE_IRI_HTTP || *other == PARTIALLY_IN_FORCE_IRI_HTTPS
	}
}
impl PartialEq<PartiallyInForceIri> for &str {
	fn eq(&self, other: &PartiallyInForceIri) -> bool {
		*self == PARTIALLY_IN_FORCE_IRI_HTTP || *self == PARTIALLY_IN_FORCE_IRI_HTTPS
	}
}
pub struct PartiallyInForceIriOrLabel;
impl PartialEq<&str> for PartiallyInForceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartiallyInForceIri || *other == PARTIALLY_IN_FORCE_LABEL
	}
}
impl PartialEq<PartiallyInForceIriOrLabel> for &str {
	fn eq(&self, other: &PartiallyInForceIriOrLabel) -> bool {
		*self == PartiallyInForceIri || *self == PARTIALLY_IN_FORCE_LABEL
	}
}
