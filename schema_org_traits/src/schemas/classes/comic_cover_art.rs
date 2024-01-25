/// <https://schema.org/ComicCoverArt>
pub trait FindComicCoverArtIds {
	type IdType;
	fn find_comic_cover_art_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindComicCoverArtIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_comic_cover_art_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COMIC_COVER_ART_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::COMIC_COVER_ART_IRI_HTTPS,
			})
		}
	}
}
