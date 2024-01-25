/// <https://schema.org/temporal>
pub const TEMPORAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/temporal";
/// <https://schema.org/temporal>
pub const TEMPORAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/temporal";
/// <https://schema.org/temporal>
pub const TEMPORAL_PROPERTY_LABEL: &str = "temporal";
pub struct TemporalPropertyIri;
impl PartialEq<&str> for TemporalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEMPORAL_PROPERTY_IRI_HTTP || *other == TEMPORAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TemporalPropertyIri> for &str {
	fn eq(&self, other: &TemporalPropertyIri) -> bool {
		*self == TEMPORAL_PROPERTY_IRI_HTTP || *self == TEMPORAL_PROPERTY_IRI_HTTPS
	}
}
pub struct TemporalPropertyIriOrLabel;
impl PartialEq<&str> for TemporalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TemporalPropertyIri || *other == TEMPORAL_PROPERTY_LABEL
	}
}
impl PartialEq<TemporalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TemporalPropertyIriOrLabel) -> bool {
		*self == TemporalPropertyIri || *self == TEMPORAL_PROPERTY_LABEL
	}
}
