/// <https://schema.org/ParcelService>
pub trait FindParcelServiceIds {
	type IdType;
	/// <https://schema.org/ParcelService>
	fn find_parcel_service_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindParcelServiceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_parcel_service_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PARCEL_SERVICE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PARCEL_SERVICE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindParcelServiceIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_parcel_service_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PARCEL_SERVICE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PARCEL_SERVICE_IRI_HTTPS,
			})
		}
	}
}
