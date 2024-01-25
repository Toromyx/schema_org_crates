/// <https://schema.org/applicableCountry>
pub const APPLICABLE_COUNTRY_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicableCountry";
/// <https://schema.org/applicableCountry>
pub const APPLICABLE_COUNTRY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicableCountry";
/// <https://schema.org/applicableCountry>
pub const APPLICABLE_COUNTRY_PROPERTY_LABEL: &str = "applicableCountry";
pub struct ApplicableCountryPropertyIri;
impl PartialEq<&str> for ApplicableCountryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICABLE_COUNTRY_PROPERTY_IRI_HTTP
			|| *other == APPLICABLE_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicableCountryPropertyIri> for &str {
	fn eq(&self, other: &ApplicableCountryPropertyIri) -> bool {
		*self == APPLICABLE_COUNTRY_PROPERTY_IRI_HTTP
			|| *self == APPLICABLE_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicableCountryPropertyIriOrLabel;
impl PartialEq<&str> for ApplicableCountryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicableCountryPropertyIri || *other == APPLICABLE_COUNTRY_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicableCountryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicableCountryPropertyIriOrLabel) -> bool {
		*self == ApplicableCountryPropertyIri || *self == APPLICABLE_COUNTRY_PROPERTY_LABEL
	}
}
