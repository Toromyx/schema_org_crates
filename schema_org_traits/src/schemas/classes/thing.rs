/// <https://schema.org/Thing>
pub trait FindThingIds {
	type IdType;
	fn find_thing_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindThingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_thing_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::THING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::THING_IRI_HTTPS,
			})
		}
	}
}
