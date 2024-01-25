/// <https://schema.org/Therapeutic>
pub const THERAPEUTIC_IRI_HTTP: &str = "http://schema.org/Therapeutic";
/// <https://schema.org/Therapeutic>
pub const THERAPEUTIC_IRI_HTTPS: &str = "https://schema.org/Therapeutic";
/// <https://schema.org/Therapeutic>
pub const THERAPEUTIC_LABEL: &str = "Therapeutic";
pub struct TherapeuticIri;
impl PartialEq<&str> for TherapeuticIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THERAPEUTIC_IRI_HTTP || *other == THERAPEUTIC_IRI_HTTPS
	}
}
impl PartialEq<TherapeuticIri> for &str {
	fn eq(&self, other: &TherapeuticIri) -> bool {
		*self == THERAPEUTIC_IRI_HTTP || *self == THERAPEUTIC_IRI_HTTPS
	}
}
pub struct TherapeuticIriOrLabel;
impl PartialEq<&str> for TherapeuticIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TherapeuticIri || *other == THERAPEUTIC_LABEL
	}
}
impl PartialEq<TherapeuticIriOrLabel> for &str {
	fn eq(&self, other: &TherapeuticIriOrLabel) -> bool {
		*self == TherapeuticIri || *self == THERAPEUTIC_LABEL
	}
}
