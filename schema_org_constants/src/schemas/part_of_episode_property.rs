/// <https://schema.org/partOfEpisode>
pub const PART_OF_EPISODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfEpisode";
/// <https://schema.org/partOfEpisode>
pub const PART_OF_EPISODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfEpisode";
/// <https://schema.org/partOfEpisode>
pub const PART_OF_EPISODE_PROPERTY_LABEL: &str = "partOfEpisode";
pub struct PartOfEpisodePropertyIri;
impl PartialEq<&str> for PartOfEpisodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_EPISODE_PROPERTY_IRI_HTTP || *other == PART_OF_EPISODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfEpisodePropertyIri> for &str {
	fn eq(&self, other: &PartOfEpisodePropertyIri) -> bool {
		*self == PART_OF_EPISODE_PROPERTY_IRI_HTTP || *self == PART_OF_EPISODE_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfEpisodePropertyIriOrLabel;
impl PartialEq<&str> for PartOfEpisodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfEpisodePropertyIri || *other == PART_OF_EPISODE_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfEpisodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfEpisodePropertyIriOrLabel) -> bool {
		*self == PartOfEpisodePropertyIri || *self == PART_OF_EPISODE_PROPERTY_LABEL
	}
}
