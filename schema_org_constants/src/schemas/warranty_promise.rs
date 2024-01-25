/// <https://schema.org/WarrantyPromise>
pub const WARRANTY_PROMISE_IRI_HTTP: &str = "http://schema.org/WarrantyPromise";
/// <https://schema.org/WarrantyPromise>
pub const WARRANTY_PROMISE_IRI_HTTPS: &str = "https://schema.org/WarrantyPromise";
/// <https://schema.org/WarrantyPromise>
pub const WARRANTY_PROMISE_LABEL: &str = "WarrantyPromise";
pub struct WarrantyPromiseIri;
impl PartialEq<&str> for WarrantyPromiseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARRANTY_PROMISE_IRI_HTTP || *other == WARRANTY_PROMISE_IRI_HTTPS
	}
}
impl PartialEq<WarrantyPromiseIri> for &str {
	fn eq(&self, other: &WarrantyPromiseIri) -> bool {
		*self == WARRANTY_PROMISE_IRI_HTTP || *self == WARRANTY_PROMISE_IRI_HTTPS
	}
}
pub struct WarrantyPromiseIriOrLabel;
impl PartialEq<&str> for WarrantyPromiseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarrantyPromiseIri || *other == WARRANTY_PROMISE_LABEL
	}
}
impl PartialEq<WarrantyPromiseIriOrLabel> for &str {
	fn eq(&self, other: &WarrantyPromiseIriOrLabel) -> bool {
		*self == WarrantyPromiseIri || *self == WARRANTY_PROMISE_LABEL
	}
}
