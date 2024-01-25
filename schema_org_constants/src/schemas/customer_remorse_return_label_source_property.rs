/// <https://schema.org/customerRemorseReturnLabelSource>
pub const CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/customerRemorseReturnLabelSource";
/// <https://schema.org/customerRemorseReturnLabelSource>
pub const CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/customerRemorseReturnLabelSource";
/// <https://schema.org/customerRemorseReturnLabelSource>
pub const CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_LABEL: &str =
	"customerRemorseReturnLabelSource";
pub struct CustomerRemorseReturnLabelSourcePropertyIri;
impl PartialEq<&str> for CustomerRemorseReturnLabelSourcePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *other == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CustomerRemorseReturnLabelSourcePropertyIri> for &str {
	fn eq(&self, other: &CustomerRemorseReturnLabelSourcePropertyIri) -> bool {
		*self == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *self == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
pub struct CustomerRemorseReturnLabelSourcePropertyIriOrLabel;
impl PartialEq<&str> for CustomerRemorseReturnLabelSourcePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CustomerRemorseReturnLabelSourcePropertyIri
			|| *other == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
impl PartialEq<CustomerRemorseReturnLabelSourcePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CustomerRemorseReturnLabelSourcePropertyIriOrLabel) -> bool {
		*self == CustomerRemorseReturnLabelSourcePropertyIri
			|| *self == CUSTOMER_REMORSE_RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
