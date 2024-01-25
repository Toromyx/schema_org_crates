/// <https://schema.org/RealEstateListing>
pub const REAL_ESTATE_LISTING_IRI_HTTP: &str = "http://schema.org/RealEstateListing";
/// <https://schema.org/RealEstateListing>
pub const REAL_ESTATE_LISTING_IRI_HTTPS: &str = "https://schema.org/RealEstateListing";
/// <https://schema.org/RealEstateListing>
pub const REAL_ESTATE_LISTING_LABEL: &str = "RealEstateListing";
pub struct RealEstateListingIri;
impl PartialEq<&str> for RealEstateListingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REAL_ESTATE_LISTING_IRI_HTTP || *other == REAL_ESTATE_LISTING_IRI_HTTPS
	}
}
impl PartialEq<RealEstateListingIri> for &str {
	fn eq(&self, other: &RealEstateListingIri) -> bool {
		*self == REAL_ESTATE_LISTING_IRI_HTTP || *self == REAL_ESTATE_LISTING_IRI_HTTPS
	}
}
pub struct RealEstateListingIriOrLabel;
impl PartialEq<&str> for RealEstateListingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RealEstateListingIri || *other == REAL_ESTATE_LISTING_LABEL
	}
}
impl PartialEq<RealEstateListingIriOrLabel> for &str {
	fn eq(&self, other: &RealEstateListingIriOrLabel) -> bool {
		*self == RealEstateListingIri || *self == REAL_ESTATE_LISTING_LABEL
	}
}
