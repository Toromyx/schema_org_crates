/// <https://schema.org/DryCleaningOrLaundry>
pub trait FindDryCleaningOrLaundryIds {
	type IdType;
	/// <https://schema.org/DryCleaningOrLaundry>
	fn find_dry_cleaning_or_laundry_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDryCleaningOrLaundryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dry_cleaning_or_laundry_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DRY_CLEANING_OR_LAUNDRY_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::DRY_CLEANING_OR_LAUNDRY_IRI_HTTPS
				}
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDryCleaningOrLaundryIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_dry_cleaning_or_laundry_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DRY_CLEANING_OR_LAUNDRY_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::DRY_CLEANING_OR_LAUNDRY_IRI_HTTPS
				}
			})
		}
	}
}
