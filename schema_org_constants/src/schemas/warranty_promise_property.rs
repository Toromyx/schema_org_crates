/// <https://schema.org/warrantyPromise>
#[deprecated = "This schema is superseded by <https://schema.org/warranty>."]
pub const WARRANTY_PROMISE_PROPERTY_IRI_HTTP: &str = "http://schema.org/warrantyPromise";
/// <https://schema.org/warrantyPromise>
#[deprecated = "This schema is superseded by <https://schema.org/warranty>."]
pub const WARRANTY_PROMISE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/warrantyPromise";
/// <https://schema.org/warrantyPromise>
#[deprecated = "This schema is superseded by <https://schema.org/warranty>."]
pub const WARRANTY_PROMISE_PROPERTY_LABEL: &str = "warrantyPromise";
pub struct WarrantyPromisePropertyIri;
impl PartialEq<&str> for WarrantyPromisePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARRANTY_PROMISE_PROPERTY_IRI_HTTP
			|| *other == WARRANTY_PROMISE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WarrantyPromisePropertyIri> for &str {
	fn eq(&self, other: &WarrantyPromisePropertyIri) -> bool {
		*self == WARRANTY_PROMISE_PROPERTY_IRI_HTTP || *self == WARRANTY_PROMISE_PROPERTY_IRI_HTTPS
	}
}
pub struct WarrantyPromisePropertyIriOrLabel;
impl PartialEq<&str> for WarrantyPromisePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarrantyPromisePropertyIri || *other == WARRANTY_PROMISE_PROPERTY_LABEL
	}
}
impl PartialEq<WarrantyPromisePropertyIriOrLabel> for &str {
	fn eq(&self, other: &WarrantyPromisePropertyIriOrLabel) -> bool {
		*self == WarrantyPromisePropertyIri || *self == WARRANTY_PROMISE_PROPERTY_LABEL
	}
}
