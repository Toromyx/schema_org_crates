/// <https://schema.org/TobaccoNicotineConsideration>
pub const TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTP: &str =
	"http://schema.org/TobaccoNicotineConsideration";
/// <https://schema.org/TobaccoNicotineConsideration>
pub const TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTPS: &str =
	"https://schema.org/TobaccoNicotineConsideration";
/// <https://schema.org/TobaccoNicotineConsideration>
pub const TOBACCO_NICOTINE_CONSIDERATION_LABEL: &str = "TobaccoNicotineConsideration";
pub struct TobaccoNicotineConsiderationIri;
impl PartialEq<&str> for TobaccoNicotineConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTP
			|| *other == TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<TobaccoNicotineConsiderationIri> for &str {
	fn eq(&self, other: &TobaccoNicotineConsiderationIri) -> bool {
		*self == TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTP
			|| *self == TOBACCO_NICOTINE_CONSIDERATION_IRI_HTTPS
	}
}
pub struct TobaccoNicotineConsiderationIriOrLabel;
impl PartialEq<&str> for TobaccoNicotineConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TobaccoNicotineConsiderationIri || *other == TOBACCO_NICOTINE_CONSIDERATION_LABEL
	}
}
impl PartialEq<TobaccoNicotineConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &TobaccoNicotineConsiderationIriOrLabel) -> bool {
		*self == TobaccoNicotineConsiderationIri || *self == TOBACCO_NICOTINE_CONSIDERATION_LABEL
	}
}
