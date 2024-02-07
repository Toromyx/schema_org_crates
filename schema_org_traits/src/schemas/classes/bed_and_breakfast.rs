/// <https://schema.org/BedAndBreakfast>
pub trait FindBedAndBreakfastIds {
	type IdType;
	/// <https://schema.org/BedAndBreakfast>
	fn find_bed_and_breakfast_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBedAndBreakfastIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bed_and_breakfast_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BED_AND_BREAKFAST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BED_AND_BREAKFAST_IRI_HTTPS,
			})
		}
	}
}
