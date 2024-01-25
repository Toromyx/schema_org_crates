/// <https://schema.org/StatisticalVariable>
pub const STATISTICAL_VARIABLE_IRI_HTTP: &str = "http://schema.org/StatisticalVariable";
/// <https://schema.org/StatisticalVariable>
pub const STATISTICAL_VARIABLE_IRI_HTTPS: &str = "https://schema.org/StatisticalVariable";
/// <https://schema.org/StatisticalVariable>
pub const STATISTICAL_VARIABLE_LABEL: &str = "StatisticalVariable";
pub struct StatisticalVariableIri;
impl PartialEq<&str> for StatisticalVariableIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATISTICAL_VARIABLE_IRI_HTTP || *other == STATISTICAL_VARIABLE_IRI_HTTPS
	}
}
impl PartialEq<StatisticalVariableIri> for &str {
	fn eq(&self, other: &StatisticalVariableIri) -> bool {
		*self == STATISTICAL_VARIABLE_IRI_HTTP || *self == STATISTICAL_VARIABLE_IRI_HTTPS
	}
}
pub struct StatisticalVariableIriOrLabel;
impl PartialEq<&str> for StatisticalVariableIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatisticalVariableIri || *other == STATISTICAL_VARIABLE_LABEL
	}
}
impl PartialEq<StatisticalVariableIriOrLabel> for &str {
	fn eq(&self, other: &StatisticalVariableIriOrLabel) -> bool {
		*self == StatisticalVariableIri || *self == STATISTICAL_VARIABLE_LABEL
	}
}
