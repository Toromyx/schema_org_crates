/// <https://schema.org/RealEstateListing>
pub trait FindRealEstateListingIds {
	type IdType;
	fn find_real_estate_listing_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRealEstateListingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_real_estate_listing_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REAL_ESTATE_LISTING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REAL_ESTATE_LISTING_IRI_HTTPS,
			})
		}
	}
}
