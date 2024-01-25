/// <https://schema.org/tongueWeight>
pub const TONGUE_WEIGHT_PROPERTY_IRI_HTTP: &str = "http://schema.org/tongueWeight";
/// <https://schema.org/tongueWeight>
pub const TONGUE_WEIGHT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tongueWeight";
/// <https://schema.org/tongueWeight>
pub const TONGUE_WEIGHT_PROPERTY_LABEL: &str = "tongueWeight";
pub struct TongueWeightPropertyIri;
impl PartialEq<&str> for TongueWeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TONGUE_WEIGHT_PROPERTY_IRI_HTTP || *other == TONGUE_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TongueWeightPropertyIri> for &str {
	fn eq(&self, other: &TongueWeightPropertyIri) -> bool {
		*self == TONGUE_WEIGHT_PROPERTY_IRI_HTTP || *self == TONGUE_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct TongueWeightPropertyIriOrLabel;
impl PartialEq<&str> for TongueWeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TongueWeightPropertyIri || *other == TONGUE_WEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<TongueWeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TongueWeightPropertyIriOrLabel) -> bool {
		*self == TongueWeightPropertyIri || *self == TONGUE_WEIGHT_PROPERTY_LABEL
	}
}
