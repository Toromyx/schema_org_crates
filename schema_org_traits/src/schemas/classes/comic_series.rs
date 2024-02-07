/// <https://schema.org/ComicSeries>
pub trait FindComicSeriesIds {
	type IdType;
	/// <https://schema.org/ComicSeries>
	fn find_comic_series_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindComicSeriesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_comic_series_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COMIC_SERIES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::COMIC_SERIES_IRI_HTTPS,
			})
		}
	}
}
