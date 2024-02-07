/// <https://schema.org/GraphicNovel>
pub trait FindGraphicNovelIds {
	type IdType;
	/// <https://schema.org/GraphicNovel>
	fn find_graphic_novel_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGraphicNovelIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_graphic_novel_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GRAPHIC_NOVEL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GRAPHIC_NOVEL_IRI_HTTPS,
			})
		}
	}
}
