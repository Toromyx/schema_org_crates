/// <https://schema.org/PodcastEpisode>
pub trait FindPodcastEpisodeIds {
	type IdType;
	/// <https://schema.org/PodcastEpisode>
	fn find_podcast_episode_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPodcastEpisodeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_podcast_episode_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PODCAST_EPISODE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PODCAST_EPISODE_IRI_HTTPS,
			})
		}
	}
}
