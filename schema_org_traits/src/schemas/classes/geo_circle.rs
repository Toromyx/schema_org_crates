/// <https://schema.org/GeoCircle>
pub trait FindGeoCircleIds {
	type IdType;
	fn find_geo_circle_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGeoCircleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_geo_circle_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GEO_CIRCLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GEO_CIRCLE_IRI_HTTPS,
			})
		}
	}
}
