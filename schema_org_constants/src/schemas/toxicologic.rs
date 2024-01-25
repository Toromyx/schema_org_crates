/// <https://schema.org/Toxicologic>
pub const TOXICOLOGIC_IRI_HTTP: &str = "http://schema.org/Toxicologic";
/// <https://schema.org/Toxicologic>
pub const TOXICOLOGIC_IRI_HTTPS: &str = "https://schema.org/Toxicologic";
/// <https://schema.org/Toxicologic>
pub const TOXICOLOGIC_LABEL: &str = "Toxicologic";
pub struct ToxicologicIri;
impl PartialEq<&str> for ToxicologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOXICOLOGIC_IRI_HTTP || *other == TOXICOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<ToxicologicIri> for &str {
	fn eq(&self, other: &ToxicologicIri) -> bool {
		*self == TOXICOLOGIC_IRI_HTTP || *self == TOXICOLOGIC_IRI_HTTPS
	}
}
pub struct ToxicologicIriOrLabel;
impl PartialEq<&str> for ToxicologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ToxicologicIri || *other == TOXICOLOGIC_LABEL
	}
}
impl PartialEq<ToxicologicIriOrLabel> for &str {
	fn eq(&self, other: &ToxicologicIriOrLabel) -> bool {
		*self == ToxicologicIri || *self == TOXICOLOGIC_LABEL
	}
}
