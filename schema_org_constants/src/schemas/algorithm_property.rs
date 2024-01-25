/// <https://schema.org/algorithm>
pub const ALGORITHM_PROPERTY_IRI_HTTP: &str = "http://schema.org/algorithm";
/// <https://schema.org/algorithm>
pub const ALGORITHM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/algorithm";
/// <https://schema.org/algorithm>
pub const ALGORITHM_PROPERTY_LABEL: &str = "algorithm";
pub struct AlgorithmPropertyIri;
impl PartialEq<&str> for AlgorithmPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALGORITHM_PROPERTY_IRI_HTTP || *other == ALGORITHM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlgorithmPropertyIri> for &str {
	fn eq(&self, other: &AlgorithmPropertyIri) -> bool {
		*self == ALGORITHM_PROPERTY_IRI_HTTP || *self == ALGORITHM_PROPERTY_IRI_HTTPS
	}
}
pub struct AlgorithmPropertyIriOrLabel;
impl PartialEq<&str> for AlgorithmPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlgorithmPropertyIri || *other == ALGORITHM_PROPERTY_LABEL
	}
}
impl PartialEq<AlgorithmPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlgorithmPropertyIriOrLabel) -> bool {
		*self == AlgorithmPropertyIri || *self == ALGORITHM_PROPERTY_LABEL
	}
}
