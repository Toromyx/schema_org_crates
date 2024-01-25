/// <https://schema.org/gracePeriod>
pub const GRACE_PERIOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/gracePeriod";
/// <https://schema.org/gracePeriod>
pub const GRACE_PERIOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gracePeriod";
/// <https://schema.org/gracePeriod>
pub const GRACE_PERIOD_PROPERTY_LABEL: &str = "gracePeriod";
pub struct GracePeriodPropertyIri;
impl PartialEq<&str> for GracePeriodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GRACE_PERIOD_PROPERTY_IRI_HTTP || *other == GRACE_PERIOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GracePeriodPropertyIri> for &str {
	fn eq(&self, other: &GracePeriodPropertyIri) -> bool {
		*self == GRACE_PERIOD_PROPERTY_IRI_HTTP || *self == GRACE_PERIOD_PROPERTY_IRI_HTTPS
	}
}
pub struct GracePeriodPropertyIriOrLabel;
impl PartialEq<&str> for GracePeriodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GracePeriodPropertyIri || *other == GRACE_PERIOD_PROPERTY_LABEL
	}
}
impl PartialEq<GracePeriodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GracePeriodPropertyIriOrLabel) -> bool {
		*self == GracePeriodPropertyIri || *self == GRACE_PERIOD_PROPERTY_LABEL
	}
}
