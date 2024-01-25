/// <https://schema.org/transitTimeLabel>
pub const TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/transitTimeLabel";
/// <https://schema.org/transitTimeLabel>
pub const TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/transitTimeLabel";
/// <https://schema.org/transitTimeLabel>
pub const TRANSIT_TIME_LABEL_PROPERTY_LABEL: &str = "transitTimeLabel";
pub struct TransitTimeLabelPropertyIri;
impl PartialEq<&str> for TransitTimeLabelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTP
			|| *other == TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TransitTimeLabelPropertyIri> for &str {
	fn eq(&self, other: &TransitTimeLabelPropertyIri) -> bool {
		*self == TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTP
			|| *self == TRANSIT_TIME_LABEL_PROPERTY_IRI_HTTPS
	}
}
pub struct TransitTimeLabelPropertyIriOrLabel;
impl PartialEq<&str> for TransitTimeLabelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransitTimeLabelPropertyIri || *other == TRANSIT_TIME_LABEL_PROPERTY_LABEL
	}
}
impl PartialEq<TransitTimeLabelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TransitTimeLabelPropertyIriOrLabel) -> bool {
		*self == TransitTimeLabelPropertyIri || *self == TRANSIT_TIME_LABEL_PROPERTY_LABEL
	}
}
