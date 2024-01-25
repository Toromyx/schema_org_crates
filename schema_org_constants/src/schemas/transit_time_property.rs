/// <https://schema.org/transitTime>
pub const TRANSIT_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/transitTime";
/// <https://schema.org/transitTime>
pub const TRANSIT_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/transitTime";
/// <https://schema.org/transitTime>
pub const TRANSIT_TIME_PROPERTY_LABEL: &str = "transitTime";
pub struct TransitTimePropertyIri;
impl PartialEq<&str> for TransitTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSIT_TIME_PROPERTY_IRI_HTTP || *other == TRANSIT_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TransitTimePropertyIri> for &str {
	fn eq(&self, other: &TransitTimePropertyIri) -> bool {
		*self == TRANSIT_TIME_PROPERTY_IRI_HTTP || *self == TRANSIT_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct TransitTimePropertyIriOrLabel;
impl PartialEq<&str> for TransitTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransitTimePropertyIri || *other == TRANSIT_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<TransitTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TransitTimePropertyIriOrLabel) -> bool {
		*self == TransitTimePropertyIri || *self == TRANSIT_TIME_PROPERTY_LABEL
	}
}
