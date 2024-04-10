/// <https://schema.org/EndorsementRating>
pub trait FindEndorsementRatingIds {
	type IdType;
	/// <https://schema.org/EndorsementRating>
	fn find_endorsement_rating_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEndorsementRatingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_endorsement_rating_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENDORSEMENT_RATING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ENDORSEMENT_RATING_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEndorsementRatingIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_endorsement_rating_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENDORSEMENT_RATING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ENDORSEMENT_RATING_IRI_HTTPS,
			})
		}
	}
}
