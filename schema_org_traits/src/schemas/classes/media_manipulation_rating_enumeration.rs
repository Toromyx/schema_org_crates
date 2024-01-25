/// <https://schema.org/MediaManipulationRatingEnumeration>
pub trait FindMediaManipulationRatingEnumerationIds {
	type IdType;
	fn find_media_manipulation_rating_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMediaManipulationRatingEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_media_manipulation_rating_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::MEDIA_MANIPULATION_RATING_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
