/// <https://schema.org/Ayurvedic>
pub const AYURVEDIC_IRI_HTTP: &str = "http://schema.org/Ayurvedic";
/// <https://schema.org/Ayurvedic>
pub const AYURVEDIC_IRI_HTTPS: &str = "https://schema.org/Ayurvedic";
/// <https://schema.org/Ayurvedic>
pub const AYURVEDIC_LABEL: &str = "Ayurvedic";
pub struct AyurvedicIri;
impl PartialEq<&str> for AyurvedicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AYURVEDIC_IRI_HTTP || *other == AYURVEDIC_IRI_HTTPS
	}
}
impl PartialEq<AyurvedicIri> for &str {
	fn eq(&self, other: &AyurvedicIri) -> bool {
		*self == AYURVEDIC_IRI_HTTP || *self == AYURVEDIC_IRI_HTTPS
	}
}
pub struct AyurvedicIriOrLabel;
impl PartialEq<&str> for AyurvedicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AyurvedicIri || *other == AYURVEDIC_LABEL
	}
}
impl PartialEq<AyurvedicIriOrLabel> for &str {
	fn eq(&self, other: &AyurvedicIriOrLabel) -> bool {
		*self == AyurvedicIri || *self == AYURVEDIC_LABEL
	}
}
