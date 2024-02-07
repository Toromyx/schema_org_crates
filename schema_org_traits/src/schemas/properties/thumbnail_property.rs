/// <https://schema.org/thumbnail>
pub trait GetThumbnailProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/thumbnail>
	fn get_thumbnail_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetThumbnailProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_thumbnail_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::THUMBNAIL_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => schema_org_constants::THUMBNAIL_PROPERTY_IRI_HTTPS,
				},
			)
		}
	}
}
