/// <https://schema.org/entertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/entertainmentBusiness";
/// <https://schema.org/entertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/entertainmentBusiness";
/// <https://schema.org/entertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_PROPERTY_LABEL: &str = "entertainmentBusiness";
pub struct EntertainmentBusinessPropertyIri;
impl PartialEq<&str> for EntertainmentBusinessPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTP
			|| *other == ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EntertainmentBusinessPropertyIri> for &str {
	fn eq(&self, other: &EntertainmentBusinessPropertyIri) -> bool {
		*self == ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTP
			|| *self == ENTERTAINMENT_BUSINESS_PROPERTY_IRI_HTTPS
	}
}
pub struct EntertainmentBusinessPropertyIriOrLabel;
impl PartialEq<&str> for EntertainmentBusinessPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EntertainmentBusinessPropertyIri
			|| *other == ENTERTAINMENT_BUSINESS_PROPERTY_LABEL
	}
}
impl PartialEq<EntertainmentBusinessPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EntertainmentBusinessPropertyIriOrLabel) -> bool {
		*self == EntertainmentBusinessPropertyIri || *self == ENTERTAINMENT_BUSINESS_PROPERTY_LABEL
	}
}
