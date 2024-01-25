/// <https://schema.org/Neuro>
pub const NEURO_IRI_HTTP: &str = "http://schema.org/Neuro";
/// <https://schema.org/Neuro>
pub const NEURO_IRI_HTTPS: &str = "https://schema.org/Neuro";
/// <https://schema.org/Neuro>
pub const NEURO_LABEL: &str = "Neuro";
pub struct NeuroIri;
impl PartialEq<&str> for NeuroIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEURO_IRI_HTTP || *other == NEURO_IRI_HTTPS
	}
}
impl PartialEq<NeuroIri> for &str {
	fn eq(&self, other: &NeuroIri) -> bool {
		*self == NEURO_IRI_HTTP || *self == NEURO_IRI_HTTPS
	}
}
pub struct NeuroIriOrLabel;
impl PartialEq<&str> for NeuroIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NeuroIri || *other == NEURO_LABEL
	}
}
impl PartialEq<NeuroIriOrLabel> for &str {
	fn eq(&self, other: &NeuroIriOrLabel) -> bool {
		*self == NeuroIri || *self == NEURO_LABEL
	}
}
