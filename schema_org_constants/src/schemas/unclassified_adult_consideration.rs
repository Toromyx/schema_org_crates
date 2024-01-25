/// <https://schema.org/UnclassifiedAdultConsideration>
pub const UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTP: &str =
	"http://schema.org/UnclassifiedAdultConsideration";
/// <https://schema.org/UnclassifiedAdultConsideration>
pub const UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTPS: &str =
	"https://schema.org/UnclassifiedAdultConsideration";
/// <https://schema.org/UnclassifiedAdultConsideration>
pub const UNCLASSIFIED_ADULT_CONSIDERATION_LABEL: &str = "UnclassifiedAdultConsideration";
pub struct UnclassifiedAdultConsiderationIri;
impl PartialEq<&str> for UnclassifiedAdultConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTP
			|| *other == UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<UnclassifiedAdultConsiderationIri> for &str {
	fn eq(&self, other: &UnclassifiedAdultConsiderationIri) -> bool {
		*self == UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTP
			|| *self == UNCLASSIFIED_ADULT_CONSIDERATION_IRI_HTTPS
	}
}
pub struct UnclassifiedAdultConsiderationIriOrLabel;
impl PartialEq<&str> for UnclassifiedAdultConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnclassifiedAdultConsiderationIri
			|| *other == UNCLASSIFIED_ADULT_CONSIDERATION_LABEL
	}
}
impl PartialEq<UnclassifiedAdultConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &UnclassifiedAdultConsiderationIriOrLabel) -> bool {
		*self == UnclassifiedAdultConsiderationIri
			|| *self == UNCLASSIFIED_ADULT_CONSIDERATION_LABEL
	}
}
