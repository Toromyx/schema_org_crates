/// <https://schema.org/restPeriods>
pub const REST_PERIODS_PROPERTY_IRI_HTTP: &str = "http://schema.org/restPeriods";
/// <https://schema.org/restPeriods>
pub const REST_PERIODS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/restPeriods";
/// <https://schema.org/restPeriods>
pub const REST_PERIODS_PROPERTY_LABEL: &str = "restPeriods";
pub struct RestPeriodsPropertyIri;
impl PartialEq<&str> for RestPeriodsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REST_PERIODS_PROPERTY_IRI_HTTP || *other == REST_PERIODS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RestPeriodsPropertyIri> for &str {
	fn eq(&self, other: &RestPeriodsPropertyIri) -> bool {
		*self == REST_PERIODS_PROPERTY_IRI_HTTP || *self == REST_PERIODS_PROPERTY_IRI_HTTPS
	}
}
pub struct RestPeriodsPropertyIriOrLabel;
impl PartialEq<&str> for RestPeriodsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RestPeriodsPropertyIri || *other == REST_PERIODS_PROPERTY_LABEL
	}
}
impl PartialEq<RestPeriodsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RestPeriodsPropertyIriOrLabel) -> bool {
		*self == RestPeriodsPropertyIri || *self == REST_PERIODS_PROPERTY_LABEL
	}
}
