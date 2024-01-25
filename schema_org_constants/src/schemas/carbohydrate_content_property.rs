/// <https://schema.org/carbohydrateContent>
pub const CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/carbohydrateContent";
/// <https://schema.org/carbohydrateContent>
pub const CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/carbohydrateContent";
/// <https://schema.org/carbohydrateContent>
pub const CARBOHYDRATE_CONTENT_PROPERTY_LABEL: &str = "carbohydrateContent";
pub struct CarbohydrateContentPropertyIri;
impl PartialEq<&str> for CarbohydrateContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTP
			|| *other == CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CarbohydrateContentPropertyIri> for &str {
	fn eq(&self, other: &CarbohydrateContentPropertyIri) -> bool {
		*self == CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTP
			|| *self == CARBOHYDRATE_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct CarbohydrateContentPropertyIriOrLabel;
impl PartialEq<&str> for CarbohydrateContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CarbohydrateContentPropertyIri || *other == CARBOHYDRATE_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<CarbohydrateContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CarbohydrateContentPropertyIriOrLabel) -> bool {
		*self == CarbohydrateContentPropertyIri || *self == CARBOHYDRATE_CONTENT_PROPERTY_LABEL
	}
}
