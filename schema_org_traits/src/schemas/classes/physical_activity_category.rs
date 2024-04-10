/// <https://schema.org/PhysicalActivityCategory>
pub trait FindPhysicalActivityCategoryIds {
	type IdType;
	/// <https://schema.org/PhysicalActivityCategory>
	fn find_physical_activity_category_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPhysicalActivityCategoryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_physical_activity_category_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTPS
				}
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPhysicalActivityCategoryIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_physical_activity_category_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTPS
				}
			})
		}
	}
}
