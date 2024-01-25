/// <https://schema.org/ReducedRelevanceForChildrenConsideration>
pub const REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTP: &str =
	"http://schema.org/ReducedRelevanceForChildrenConsideration";
/// <https://schema.org/ReducedRelevanceForChildrenConsideration>
pub const REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTPS: &str =
	"https://schema.org/ReducedRelevanceForChildrenConsideration";
/// <https://schema.org/ReducedRelevanceForChildrenConsideration>
pub const REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_LABEL: &str =
	"ReducedRelevanceForChildrenConsideration";
pub struct ReducedRelevanceForChildrenConsiderationIri;
impl PartialEq<&str> for ReducedRelevanceForChildrenConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTP
			|| *other == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<ReducedRelevanceForChildrenConsiderationIri> for &str {
	fn eq(&self, other: &ReducedRelevanceForChildrenConsiderationIri) -> bool {
		*self == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTP
			|| *self == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_IRI_HTTPS
	}
}
pub struct ReducedRelevanceForChildrenConsiderationIriOrLabel;
impl PartialEq<&str> for ReducedRelevanceForChildrenConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReducedRelevanceForChildrenConsiderationIri
			|| *other == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_LABEL
	}
}
impl PartialEq<ReducedRelevanceForChildrenConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &ReducedRelevanceForChildrenConsiderationIriOrLabel) -> bool {
		*self == ReducedRelevanceForChildrenConsiderationIri
			|| *self == REDUCED_RELEVANCE_FOR_CHILDREN_CONSIDERATION_LABEL
	}
}
