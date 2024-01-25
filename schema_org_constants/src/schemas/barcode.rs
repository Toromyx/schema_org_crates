/// <https://schema.org/Barcode>
pub const BARCODE_IRI_HTTP: &str = "http://schema.org/Barcode";
/// <https://schema.org/Barcode>
pub const BARCODE_IRI_HTTPS: &str = "https://schema.org/Barcode";
/// <https://schema.org/Barcode>
pub const BARCODE_LABEL: &str = "Barcode";
pub struct BarcodeIri;
impl PartialEq<&str> for BarcodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BARCODE_IRI_HTTP || *other == BARCODE_IRI_HTTPS
	}
}
impl PartialEq<BarcodeIri> for &str {
	fn eq(&self, other: &BarcodeIri) -> bool {
		*self == BARCODE_IRI_HTTP || *self == BARCODE_IRI_HTTPS
	}
}
pub struct BarcodeIriOrLabel;
impl PartialEq<&str> for BarcodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BarcodeIri || *other == BARCODE_LABEL
	}
}
impl PartialEq<BarcodeIriOrLabel> for &str {
	fn eq(&self, other: &BarcodeIriOrLabel) -> bool {
		*self == BarcodeIri || *self == BARCODE_LABEL
	}
}
