/// <https://schema.org/WPAdBlock>
pub trait FindWpAdBlockIds {
	type IdType;
	fn find_wp_ad_block_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWpAdBlockIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wp_ad_block_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WP_AD_BLOCK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WP_AD_BLOCK_IRI_HTTPS,
			})
		}
	}
}
