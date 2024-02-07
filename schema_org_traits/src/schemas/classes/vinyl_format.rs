/// <https://schema.org/VinylFormat>
pub trait FindVinylFormatIds {
	type IdType;
	/// <https://schema.org/VinylFormat>
	fn find_vinyl_format_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVinylFormatIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_vinyl_format_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VINYL_FORMAT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VINYL_FORMAT_IRI_HTTPS,
			})
		}
	}
}
