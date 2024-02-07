/// <https://schema.org/OfferForLease>
pub trait FindOfferForLeaseIds {
	type IdType;
	/// <https://schema.org/OfferForLease>
	fn find_offer_for_lease_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOfferForLeaseIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_offer_for_lease_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OFFER_FOR_LEASE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OFFER_FOR_LEASE_IRI_HTTPS,
			})
		}
	}
}
