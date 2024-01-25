/// <https://schema.org/HowToTip>
pub const HOW_TO_TIP_IRI_HTTP: &str = "http://schema.org/HowToTip";
/// <https://schema.org/HowToTip>
pub const HOW_TO_TIP_IRI_HTTPS: &str = "https://schema.org/HowToTip";
/// <https://schema.org/HowToTip>
pub const HOW_TO_TIP_LABEL: &str = "HowToTip";
pub struct HowToTipIri;
impl PartialEq<&str> for HowToTipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_TIP_IRI_HTTP || *other == HOW_TO_TIP_IRI_HTTPS
	}
}
impl PartialEq<HowToTipIri> for &str {
	fn eq(&self, other: &HowToTipIri) -> bool {
		*self == HOW_TO_TIP_IRI_HTTP || *self == HOW_TO_TIP_IRI_HTTPS
	}
}
pub struct HowToTipIriOrLabel;
impl PartialEq<&str> for HowToTipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToTipIri || *other == HOW_TO_TIP_LABEL
	}
}
impl PartialEq<HowToTipIriOrLabel> for &str {
	fn eq(&self, other: &HowToTipIriOrLabel) -> bool {
		*self == HowToTipIri || *self == HOW_TO_TIP_LABEL
	}
}
