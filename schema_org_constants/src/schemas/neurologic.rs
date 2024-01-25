/// <https://schema.org/Neurologic>
pub const NEUROLOGIC_IRI_HTTP: &str = "http://schema.org/Neurologic";
/// <https://schema.org/Neurologic>
pub const NEUROLOGIC_IRI_HTTPS: &str = "https://schema.org/Neurologic";
/// <https://schema.org/Neurologic>
pub const NEUROLOGIC_LABEL: &str = "Neurologic";
pub struct NeurologicIri;
impl PartialEq<&str> for NeurologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEUROLOGIC_IRI_HTTP || *other == NEUROLOGIC_IRI_HTTPS
	}
}
impl PartialEq<NeurologicIri> for &str {
	fn eq(&self, other: &NeurologicIri) -> bool {
		*self == NEUROLOGIC_IRI_HTTP || *self == NEUROLOGIC_IRI_HTTPS
	}
}
pub struct NeurologicIriOrLabel;
impl PartialEq<&str> for NeurologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NeurologicIri || *other == NEUROLOGIC_LABEL
	}
}
impl PartialEq<NeurologicIriOrLabel> for &str {
	fn eq(&self, other: &NeurologicIriOrLabel) -> bool {
		*self == NeurologicIri || *self == NEUROLOGIC_LABEL
	}
}
