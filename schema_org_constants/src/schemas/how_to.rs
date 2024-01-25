/// <https://schema.org/HowTo>
pub const HOW_TO_IRI_HTTP: &str = "http://schema.org/HowTo";
/// <https://schema.org/HowTo>
pub const HOW_TO_IRI_HTTPS: &str = "https://schema.org/HowTo";
/// <https://schema.org/HowTo>
pub const HOW_TO_LABEL: &str = "HowTo";
pub struct HowToIri;
impl PartialEq<&str> for HowToIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_IRI_HTTP || *other == HOW_TO_IRI_HTTPS
	}
}
impl PartialEq<HowToIri> for &str {
	fn eq(&self, other: &HowToIri) -> bool {
		*self == HOW_TO_IRI_HTTP || *self == HOW_TO_IRI_HTTPS
	}
}
pub struct HowToIriOrLabel;
impl PartialEq<&str> for HowToIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToIri || *other == HOW_TO_LABEL
	}
}
impl PartialEq<HowToIriOrLabel> for &str {
	fn eq(&self, other: &HowToIriOrLabel) -> bool {
		*self == HowToIri || *self == HOW_TO_LABEL
	}
}
