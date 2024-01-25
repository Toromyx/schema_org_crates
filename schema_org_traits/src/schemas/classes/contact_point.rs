/// <https://schema.org/ContactPoint>
pub trait FindContactPointIds {
	type IdType;
	fn find_contact_point_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindContactPointIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_contact_point_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CONTACT_POINT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CONTACT_POINT_IRI_HTTPS,
			})
		}
	}
}
