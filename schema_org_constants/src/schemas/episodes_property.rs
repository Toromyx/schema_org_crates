/// <https://schema.org/episodes>
#[deprecated = "This schema is superseded by <https://schema.org/episode>."]
pub const EPISODES_PROPERTY_IRI_HTTP: &str = "http://schema.org/episodes";
/// <https://schema.org/episodes>
#[deprecated = "This schema is superseded by <https://schema.org/episode>."]
pub const EPISODES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/episodes";
/// <https://schema.org/episodes>
#[deprecated = "This schema is superseded by <https://schema.org/episode>."]
pub const EPISODES_PROPERTY_LABEL: &str = "episodes";
pub struct EpisodesPropertyIri;
impl PartialEq<&str> for EpisodesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EPISODES_PROPERTY_IRI_HTTP || *other == EPISODES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EpisodesPropertyIri> for &str {
	fn eq(&self, other: &EpisodesPropertyIri) -> bool {
		*self == EPISODES_PROPERTY_IRI_HTTP || *self == EPISODES_PROPERTY_IRI_HTTPS
	}
}
pub struct EpisodesPropertyIriOrLabel;
impl PartialEq<&str> for EpisodesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpisodesPropertyIri || *other == EPISODES_PROPERTY_LABEL
	}
}
impl PartialEq<EpisodesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EpisodesPropertyIriOrLabel) -> bool {
		*self == EpisodesPropertyIri || *self == EPISODES_PROPERTY_LABEL
	}
}
