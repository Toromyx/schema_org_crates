/// <https://schema.org/originalMediaContextDescription>
pub trait GetOriginalMediaContextDescriptionProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/originalMediaContextDescription>
	fn get_original_media_context_description_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetOriginalMediaContextDescriptionProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_original_media_context_description_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
