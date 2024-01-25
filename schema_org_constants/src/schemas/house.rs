/// <https://schema.org/House>
pub const HOUSE_IRI_HTTP: &str = "http://schema.org/House";
/// <https://schema.org/House>
pub const HOUSE_IRI_HTTPS: &str = "https://schema.org/House";
/// <https://schema.org/House>
pub const HOUSE_LABEL: &str = "House";
pub struct HouseIri;
impl PartialEq<&str> for HouseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOUSE_IRI_HTTP || *other == HOUSE_IRI_HTTPS
	}
}
impl PartialEq<HouseIri> for &str {
	fn eq(&self, other: &HouseIri) -> bool {
		*self == HOUSE_IRI_HTTP || *self == HOUSE_IRI_HTTPS
	}
}
pub struct HouseIriOrLabel;
impl PartialEq<&str> for HouseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HouseIri || *other == HOUSE_LABEL
	}
}
impl PartialEq<HouseIriOrLabel> for &str {
	fn eq(&self, other: &HouseIriOrLabel) -> bool {
		*self == HouseIri || *self == HOUSE_LABEL
	}
}
