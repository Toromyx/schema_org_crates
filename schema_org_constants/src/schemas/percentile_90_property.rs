/// <https://schema.org/percentile90>
pub const PERCENTILE_90_PROPERTY_IRI_HTTP: &str = "http://schema.org/percentile90";
/// <https://schema.org/percentile90>
pub const PERCENTILE_90_PROPERTY_IRI_HTTPS: &str = "https://schema.org/percentile90";
/// <https://schema.org/percentile90>
pub const PERCENTILE_90_PROPERTY_LABEL: &str = "percentile90";
pub struct Percentile90PropertyIri;
impl PartialEq<&str> for Percentile90PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERCENTILE_90_PROPERTY_IRI_HTTP || *other == PERCENTILE_90_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Percentile90PropertyIri> for &str {
	fn eq(&self, other: &Percentile90PropertyIri) -> bool {
		*self == PERCENTILE_90_PROPERTY_IRI_HTTP || *self == PERCENTILE_90_PROPERTY_IRI_HTTPS
	}
}
pub struct Percentile90PropertyIriOrLabel;
impl PartialEq<&str> for Percentile90PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Percentile90PropertyIri || *other == PERCENTILE_90_PROPERTY_LABEL
	}
}
impl PartialEq<Percentile90PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Percentile90PropertyIriOrLabel) -> bool {
		*self == Percentile90PropertyIri || *self == PERCENTILE_90_PROPERTY_LABEL
	}
}
