/// <https://schema.org/InForce>
pub const IN_FORCE_IRI_HTTP: &str = "http://schema.org/InForce";
/// <https://schema.org/InForce>
pub const IN_FORCE_IRI_HTTPS: &str = "https://schema.org/InForce";
/// <https://schema.org/InForce>
pub const IN_FORCE_LABEL: &str = "InForce";
pub struct InForceIri;
impl PartialEq<&str> for InForceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_FORCE_IRI_HTTP || *other == IN_FORCE_IRI_HTTPS
	}
}
impl PartialEq<InForceIri> for &str {
	fn eq(&self, other: &InForceIri) -> bool {
		*self == IN_FORCE_IRI_HTTP || *self == IN_FORCE_IRI_HTTPS
	}
}
pub struct InForceIriOrLabel;
impl PartialEq<&str> for InForceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InForceIri || *other == IN_FORCE_LABEL
	}
}
impl PartialEq<InForceIriOrLabel> for &str {
	fn eq(&self, other: &InForceIriOrLabel) -> bool {
		*self == InForceIri || *self == IN_FORCE_LABEL
	}
}
