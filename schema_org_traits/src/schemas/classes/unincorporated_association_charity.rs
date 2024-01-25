/// <https://schema.org/UnincorporatedAssociationCharity>
pub trait FindUnincorporatedAssociationCharityIds {
	type IdType;
	fn find_unincorporated_association_charity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUnincorporatedAssociationCharityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_unincorporated_association_charity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTPS
				}
			})
		}
	}
}
