/// <https://schema.org/Restaurant>
pub const RESTAURANT_IRI_HTTP: &str = "http://schema.org/Restaurant";
/// <https://schema.org/Restaurant>
pub const RESTAURANT_IRI_HTTPS: &str = "https://schema.org/Restaurant";
/// <https://schema.org/Restaurant>
pub const RESTAURANT_LABEL: &str = "Restaurant";
pub struct RestaurantIri;
impl PartialEq<&str> for RestaurantIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESTAURANT_IRI_HTTP || *other == RESTAURANT_IRI_HTTPS
	}
}
impl PartialEq<RestaurantIri> for &str {
	fn eq(&self, other: &RestaurantIri) -> bool {
		*self == RESTAURANT_IRI_HTTP || *self == RESTAURANT_IRI_HTTPS
	}
}
pub struct RestaurantIriOrLabel;
impl PartialEq<&str> for RestaurantIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RestaurantIri || *other == RESTAURANT_LABEL
	}
}
impl PartialEq<RestaurantIriOrLabel> for &str {
	fn eq(&self, other: &RestaurantIriOrLabel) -> bool {
		*self == RestaurantIri || *self == RESTAURANT_LABEL
	}
}
