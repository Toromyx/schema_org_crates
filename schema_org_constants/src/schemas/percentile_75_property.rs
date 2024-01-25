/// <https://schema.org/percentile75>
pub const PERCENTILE_75_PROPERTY_IRI_HTTP: &str = "http://schema.org/percentile75";
/// <https://schema.org/percentile75>
pub const PERCENTILE_75_PROPERTY_IRI_HTTPS: &str = "https://schema.org/percentile75";
/// <https://schema.org/percentile75>
pub const PERCENTILE_75_PROPERTY_LABEL: &str = "percentile75";
pub struct Percentile75PropertyIri;
impl PartialEq<&str> for Percentile75PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERCENTILE_75_PROPERTY_IRI_HTTP || *other == PERCENTILE_75_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Percentile75PropertyIri> for &str {
	fn eq(&self, other: &Percentile75PropertyIri) -> bool {
		*self == PERCENTILE_75_PROPERTY_IRI_HTTP || *self == PERCENTILE_75_PROPERTY_IRI_HTTPS
	}
}
pub struct Percentile75PropertyIriOrLabel;
impl PartialEq<&str> for Percentile75PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Percentile75PropertyIri || *other == PERCENTILE_75_PROPERTY_LABEL
	}
}
impl PartialEq<Percentile75PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Percentile75PropertyIriOrLabel) -> bool {
		*self == Percentile75PropertyIri || *self == PERCENTILE_75_PROPERTY_LABEL
	}
}
