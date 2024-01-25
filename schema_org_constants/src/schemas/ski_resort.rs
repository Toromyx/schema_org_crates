/// <https://schema.org/SkiResort>
pub const SKI_RESORT_IRI_HTTP: &str = "http://schema.org/SkiResort";
/// <https://schema.org/SkiResort>
pub const SKI_RESORT_IRI_HTTPS: &str = "https://schema.org/SkiResort";
/// <https://schema.org/SkiResort>
pub const SKI_RESORT_LABEL: &str = "SkiResort";
pub struct SkiResortIri;
impl PartialEq<&str> for SkiResortIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SKI_RESORT_IRI_HTTP || *other == SKI_RESORT_IRI_HTTPS
	}
}
impl PartialEq<SkiResortIri> for &str {
	fn eq(&self, other: &SkiResortIri) -> bool {
		*self == SKI_RESORT_IRI_HTTP || *self == SKI_RESORT_IRI_HTTPS
	}
}
pub struct SkiResortIriOrLabel;
impl PartialEq<&str> for SkiResortIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SkiResortIri || *other == SKI_RESORT_LABEL
	}
}
impl PartialEq<SkiResortIriOrLabel> for &str {
	fn eq(&self, other: &SkiResortIriOrLabel) -> bool {
		*self == SkiResortIri || *self == SKI_RESORT_LABEL
	}
}
