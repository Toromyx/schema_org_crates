/// <https://schema.org/RoofingContractor>
pub trait FindRoofingContractorIds {
	type IdType;
	/// <https://schema.org/RoofingContractor>
	fn find_roofing_contractor_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRoofingContractorIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_roofing_contractor_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ROOFING_CONTRACTOR_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ROOFING_CONTRACTOR_IRI_HTTPS,
			})
		}
	}
}
