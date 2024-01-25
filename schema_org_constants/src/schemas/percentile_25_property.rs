/// <https://schema.org/percentile25>
pub const PERCENTILE_25_PROPERTY_IRI_HTTP: &str = "http://schema.org/percentile25";
/// <https://schema.org/percentile25>
pub const PERCENTILE_25_PROPERTY_IRI_HTTPS: &str = "https://schema.org/percentile25";
/// <https://schema.org/percentile25>
pub const PERCENTILE_25_PROPERTY_LABEL: &str = "percentile25";
pub struct Percentile25PropertyIri;
impl PartialEq<&str> for Percentile25PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERCENTILE_25_PROPERTY_IRI_HTTP || *other == PERCENTILE_25_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Percentile25PropertyIri> for &str {
	fn eq(&self, other: &Percentile25PropertyIri) -> bool {
		*self == PERCENTILE_25_PROPERTY_IRI_HTTP || *self == PERCENTILE_25_PROPERTY_IRI_HTTPS
	}
}
pub struct Percentile25PropertyIriOrLabel;
impl PartialEq<&str> for Percentile25PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Percentile25PropertyIri || *other == PERCENTILE_25_PROPERTY_LABEL
	}
}
impl PartialEq<Percentile25PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Percentile25PropertyIriOrLabel) -> bool {
		*self == Percentile25PropertyIri || *self == PERCENTILE_25_PROPERTY_LABEL
	}
}
