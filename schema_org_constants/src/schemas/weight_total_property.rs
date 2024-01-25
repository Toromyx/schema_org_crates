/// <https://schema.org/weightTotal>
pub const WEIGHT_TOTAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/weightTotal";
/// <https://schema.org/weightTotal>
pub const WEIGHT_TOTAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/weightTotal";
/// <https://schema.org/weightTotal>
pub const WEIGHT_TOTAL_PROPERTY_LABEL: &str = "weightTotal";
pub struct WeightTotalPropertyIri;
impl PartialEq<&str> for WeightTotalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEIGHT_TOTAL_PROPERTY_IRI_HTTP || *other == WEIGHT_TOTAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WeightTotalPropertyIri> for &str {
	fn eq(&self, other: &WeightTotalPropertyIri) -> bool {
		*self == WEIGHT_TOTAL_PROPERTY_IRI_HTTP || *self == WEIGHT_TOTAL_PROPERTY_IRI_HTTPS
	}
}
pub struct WeightTotalPropertyIriOrLabel;
impl PartialEq<&str> for WeightTotalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WeightTotalPropertyIri || *other == WEIGHT_TOTAL_PROPERTY_LABEL
	}
}
impl PartialEq<WeightTotalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WeightTotalPropertyIriOrLabel) -> bool {
		*self == WeightTotalPropertyIri || *self == WEIGHT_TOTAL_PROPERTY_LABEL
	}
}
