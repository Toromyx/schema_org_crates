/// <https://schema.org/AutoDealer>
pub trait FindAutoDealerIds {
	type IdType;
	/// <https://schema.org/AutoDealer>
	fn find_auto_dealer_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAutoDealerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_auto_dealer_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AUTO_DEALER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AUTO_DEALER_IRI_HTTPS,
			})
		}
	}
}
