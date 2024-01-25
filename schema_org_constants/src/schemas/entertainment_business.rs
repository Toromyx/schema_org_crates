/// <https://schema.org/EntertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_IRI_HTTP: &str = "http://schema.org/EntertainmentBusiness";
/// <https://schema.org/EntertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_IRI_HTTPS: &str = "https://schema.org/EntertainmentBusiness";
/// <https://schema.org/EntertainmentBusiness>
pub const ENTERTAINMENT_BUSINESS_LABEL: &str = "EntertainmentBusiness";
pub struct EntertainmentBusinessIri;
impl PartialEq<&str> for EntertainmentBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENTERTAINMENT_BUSINESS_IRI_HTTP || *other == ENTERTAINMENT_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<EntertainmentBusinessIri> for &str {
	fn eq(&self, other: &EntertainmentBusinessIri) -> bool {
		*self == ENTERTAINMENT_BUSINESS_IRI_HTTP || *self == ENTERTAINMENT_BUSINESS_IRI_HTTPS
	}
}
pub struct EntertainmentBusinessIriOrLabel;
impl PartialEq<&str> for EntertainmentBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EntertainmentBusinessIri || *other == ENTERTAINMENT_BUSINESS_LABEL
	}
}
impl PartialEq<EntertainmentBusinessIriOrLabel> for &str {
	fn eq(&self, other: &EntertainmentBusinessIriOrLabel) -> bool {
		*self == EntertainmentBusinessIri || *self == ENTERTAINMENT_BUSINESS_LABEL
	}
}
