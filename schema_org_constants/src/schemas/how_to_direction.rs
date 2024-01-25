/// <https://schema.org/HowToDirection>
pub const HOW_TO_DIRECTION_IRI_HTTP: &str = "http://schema.org/HowToDirection";
/// <https://schema.org/HowToDirection>
pub const HOW_TO_DIRECTION_IRI_HTTPS: &str = "https://schema.org/HowToDirection";
/// <https://schema.org/HowToDirection>
pub const HOW_TO_DIRECTION_LABEL: &str = "HowToDirection";
pub struct HowToDirectionIri;
impl PartialEq<&str> for HowToDirectionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_DIRECTION_IRI_HTTP || *other == HOW_TO_DIRECTION_IRI_HTTPS
	}
}
impl PartialEq<HowToDirectionIri> for &str {
	fn eq(&self, other: &HowToDirectionIri) -> bool {
		*self == HOW_TO_DIRECTION_IRI_HTTP || *self == HOW_TO_DIRECTION_IRI_HTTPS
	}
}
pub struct HowToDirectionIriOrLabel;
impl PartialEq<&str> for HowToDirectionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToDirectionIri || *other == HOW_TO_DIRECTION_LABEL
	}
}
impl PartialEq<HowToDirectionIriOrLabel> for &str {
	fn eq(&self, other: &HowToDirectionIriOrLabel) -> bool {
		*self == HowToDirectionIri || *self == HOW_TO_DIRECTION_LABEL
	}
}
