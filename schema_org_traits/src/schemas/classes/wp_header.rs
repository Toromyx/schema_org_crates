/// <https://schema.org/WPHeader>
pub trait FindWpHeaderIds {
	type IdType;
	fn find_wp_header_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWpHeaderIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wp_header_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WP_HEADER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WP_HEADER_IRI_HTTPS,
			})
		}
	}
}
