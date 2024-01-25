/// <https://schema.org/HomeAndConstructionBusiness>
pub const HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTP: &str =
	"http://schema.org/HomeAndConstructionBusiness";
/// <https://schema.org/HomeAndConstructionBusiness>
pub const HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTPS: &str =
	"https://schema.org/HomeAndConstructionBusiness";
/// <https://schema.org/HomeAndConstructionBusiness>
pub const HOME_AND_CONSTRUCTION_BUSINESS_LABEL: &str = "HomeAndConstructionBusiness";
pub struct HomeAndConstructionBusinessIri;
impl PartialEq<&str> for HomeAndConstructionBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTP
			|| *other == HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<HomeAndConstructionBusinessIri> for &str {
	fn eq(&self, other: &HomeAndConstructionBusinessIri) -> bool {
		*self == HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTP
			|| *self == HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTPS
	}
}
pub struct HomeAndConstructionBusinessIriOrLabel;
impl PartialEq<&str> for HomeAndConstructionBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HomeAndConstructionBusinessIri || *other == HOME_AND_CONSTRUCTION_BUSINESS_LABEL
	}
}
impl PartialEq<HomeAndConstructionBusinessIriOrLabel> for &str {
	fn eq(&self, other: &HomeAndConstructionBusinessIriOrLabel) -> bool {
		*self == HomeAndConstructionBusinessIri || *self == HOME_AND_CONSTRUCTION_BUSINESS_LABEL
	}
}
