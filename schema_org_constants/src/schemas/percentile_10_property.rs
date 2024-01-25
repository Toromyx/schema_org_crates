/// <https://schema.org/percentile10>
pub const PERCENTILE_10_PROPERTY_IRI_HTTP: &str = "http://schema.org/percentile10";
/// <https://schema.org/percentile10>
pub const PERCENTILE_10_PROPERTY_IRI_HTTPS: &str = "https://schema.org/percentile10";
/// <https://schema.org/percentile10>
pub const PERCENTILE_10_PROPERTY_LABEL: &str = "percentile10";
pub struct Percentile10PropertyIri;
impl PartialEq<&str> for Percentile10PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERCENTILE_10_PROPERTY_IRI_HTTP || *other == PERCENTILE_10_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Percentile10PropertyIri> for &str {
	fn eq(&self, other: &Percentile10PropertyIri) -> bool {
		*self == PERCENTILE_10_PROPERTY_IRI_HTTP || *self == PERCENTILE_10_PROPERTY_IRI_HTTPS
	}
}
pub struct Percentile10PropertyIriOrLabel;
impl PartialEq<&str> for Percentile10PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Percentile10PropertyIri || *other == PERCENTILE_10_PROPERTY_LABEL
	}
}
impl PartialEq<Percentile10PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Percentile10PropertyIriOrLabel) -> bool {
		*self == Percentile10PropertyIri || *self == PERCENTILE_10_PROPERTY_LABEL
	}
}
