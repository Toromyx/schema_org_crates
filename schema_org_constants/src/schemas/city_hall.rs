/// <https://schema.org/CityHall>
pub const CITY_HALL_IRI_HTTP: &str = "http://schema.org/CityHall";
/// <https://schema.org/CityHall>
pub const CITY_HALL_IRI_HTTPS: &str = "https://schema.org/CityHall";
/// <https://schema.org/CityHall>
pub const CITY_HALL_LABEL: &str = "CityHall";
pub struct CityHallIri;
impl PartialEq<&str> for CityHallIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CITY_HALL_IRI_HTTP || *other == CITY_HALL_IRI_HTTPS
	}
}
impl PartialEq<CityHallIri> for &str {
	fn eq(&self, other: &CityHallIri) -> bool {
		*self == CITY_HALL_IRI_HTTP || *self == CITY_HALL_IRI_HTTPS
	}
}
pub struct CityHallIriOrLabel;
impl PartialEq<&str> for CityHallIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CityHallIri || *other == CITY_HALL_LABEL
	}
}
impl PartialEq<CityHallIriOrLabel> for &str {
	fn eq(&self, other: &CityHallIriOrLabel) -> bool {
		*self == CityHallIri || *self == CITY_HALL_LABEL
	}
}
