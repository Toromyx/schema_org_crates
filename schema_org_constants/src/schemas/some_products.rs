/// <https://schema.org/SomeProducts>
pub const SOME_PRODUCTS_IRI_HTTP: &str = "http://schema.org/SomeProducts";
/// <https://schema.org/SomeProducts>
pub const SOME_PRODUCTS_IRI_HTTPS: &str = "https://schema.org/SomeProducts";
/// <https://schema.org/SomeProducts>
pub const SOME_PRODUCTS_LABEL: &str = "SomeProducts";
pub struct SomeProductsIri;
impl PartialEq<&str> for SomeProductsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOME_PRODUCTS_IRI_HTTP || *other == SOME_PRODUCTS_IRI_HTTPS
	}
}
impl PartialEq<SomeProductsIri> for &str {
	fn eq(&self, other: &SomeProductsIri) -> bool {
		*self == SOME_PRODUCTS_IRI_HTTP || *self == SOME_PRODUCTS_IRI_HTTPS
	}
}
pub struct SomeProductsIriOrLabel;
impl PartialEq<&str> for SomeProductsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SomeProductsIri || *other == SOME_PRODUCTS_LABEL
	}
}
impl PartialEq<SomeProductsIriOrLabel> for &str {
	fn eq(&self, other: &SomeProductsIriOrLabel) -> bool {
		*self == SomeProductsIri || *self == SOME_PRODUCTS_LABEL
	}
}
