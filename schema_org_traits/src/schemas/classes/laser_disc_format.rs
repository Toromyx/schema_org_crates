/// <https://schema.org/LaserDiscFormat>
pub trait FindLaserDiscFormatIds {
	type IdType;
	/// <https://schema.org/LaserDiscFormat>
	fn find_laser_disc_format_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLaserDiscFormatIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_laser_disc_format_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LASER_DISC_FORMAT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LASER_DISC_FORMAT_IRI_HTTPS,
			})
		}
	}
}
