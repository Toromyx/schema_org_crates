/// <https://schema.org/repetitions>
pub const REPETITIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/repetitions";
/// <https://schema.org/repetitions>
pub const REPETITIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/repetitions";
/// <https://schema.org/repetitions>
pub const REPETITIONS_PROPERTY_LABEL: &str = "repetitions";
pub struct RepetitionsPropertyIri;
impl PartialEq<&str> for RepetitionsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPETITIONS_PROPERTY_IRI_HTTP || *other == REPETITIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RepetitionsPropertyIri> for &str {
	fn eq(&self, other: &RepetitionsPropertyIri) -> bool {
		*self == REPETITIONS_PROPERTY_IRI_HTTP || *self == REPETITIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct RepetitionsPropertyIriOrLabel;
impl PartialEq<&str> for RepetitionsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RepetitionsPropertyIri || *other == REPETITIONS_PROPERTY_LABEL
	}
}
impl PartialEq<RepetitionsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RepetitionsPropertyIriOrLabel) -> bool {
		*self == RepetitionsPropertyIri || *self == REPETITIONS_PROPERTY_LABEL
	}
}
