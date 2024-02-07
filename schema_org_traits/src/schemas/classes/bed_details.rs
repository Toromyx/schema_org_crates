/// <https://schema.org/BedDetails>
pub trait FindBedDetailsIds {
	type IdType;
	/// <https://schema.org/BedDetails>
	fn find_bed_details_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBedDetailsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bed_details_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BED_DETAILS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BED_DETAILS_IRI_HTTPS,
			})
		}
	}
}
