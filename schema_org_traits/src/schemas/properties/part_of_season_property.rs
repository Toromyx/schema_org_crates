/// <https://schema.org/partOfSeason>
pub trait GetPartOfSeasonProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/partOfSeason>
	fn get_part_of_season_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetPartOfSeasonProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_part_of_season_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::PART_OF_SEASON_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::PART_OF_SEASON_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
