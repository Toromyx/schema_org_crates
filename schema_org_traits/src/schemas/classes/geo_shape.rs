/// <https://schema.org/GeoShape>
pub trait FindGeoShapeIds {
	type IdType;
	fn find_geo_shape_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGeoShapeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_geo_shape_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GEO_SHAPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GEO_SHAPE_IRI_HTTPS,
			})
		}
	}
}
