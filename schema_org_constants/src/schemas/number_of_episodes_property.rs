/// <https://schema.org/numberOfEpisodes>
pub const NUMBER_OF_EPISODES_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfEpisodes";
/// <https://schema.org/numberOfEpisodes>
pub const NUMBER_OF_EPISODES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfEpisodes";
/// <https://schema.org/numberOfEpisodes>
pub const NUMBER_OF_EPISODES_PROPERTY_LABEL: &str = "numberOfEpisodes";
pub struct NumberOfEpisodesPropertyIri;
impl PartialEq<&str> for NumberOfEpisodesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_EPISODES_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_EPISODES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfEpisodesPropertyIri> for &str {
	fn eq(&self, other: &NumberOfEpisodesPropertyIri) -> bool {
		*self == NUMBER_OF_EPISODES_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_EPISODES_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfEpisodesPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfEpisodesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfEpisodesPropertyIri || *other == NUMBER_OF_EPISODES_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfEpisodesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfEpisodesPropertyIriOrLabel) -> bool {
		*self == NumberOfEpisodesPropertyIri || *self == NUMBER_OF_EPISODES_PROPERTY_LABEL
	}
}
