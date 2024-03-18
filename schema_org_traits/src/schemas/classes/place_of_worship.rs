/// <https://schema.org/PlaceOfWorship>
pub trait FindPlaceOfWorshipIds {
	type IdType;
	/// <https://schema.org/PlaceOfWorship>
	fn find_place_of_worship_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPlaceOfWorshipIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_place_of_worship_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PLACE_OF_WORSHIP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PLACE_OF_WORSHIP_IRI_HTTPS,
			})
		}
	}
}