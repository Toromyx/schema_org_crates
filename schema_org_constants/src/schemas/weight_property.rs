/// <https://schema.org/weight>
pub const WEIGHT_PROPERTY_IRI_HTTP: &str = "http://schema.org/weight";
/// <https://schema.org/weight>
pub const WEIGHT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/weight";
/// <https://schema.org/weight>
pub const WEIGHT_PROPERTY_LABEL: &str = "weight";
pub struct WeightPropertyIri;
impl PartialEq<&str> for WeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEIGHT_PROPERTY_IRI_HTTP || *other == WEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WeightPropertyIri> for &str {
	fn eq(&self, other: &WeightPropertyIri) -> bool {
		*self == WEIGHT_PROPERTY_IRI_HTTP || *self == WEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct WeightPropertyIriOrLabel;
impl PartialEq<&str> for WeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WeightPropertyIri || *other == WEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<WeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WeightPropertyIriOrLabel) -> bool {
		*self == WeightPropertyIri || *self == WEIGHT_PROPERTY_LABEL
	}
}
