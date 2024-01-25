/// <https://schema.org/BorrowAction>
pub const BORROW_ACTION_IRI_HTTP: &str = "http://schema.org/BorrowAction";
/// <https://schema.org/BorrowAction>
pub const BORROW_ACTION_IRI_HTTPS: &str = "https://schema.org/BorrowAction";
/// <https://schema.org/BorrowAction>
pub const BORROW_ACTION_LABEL: &str = "BorrowAction";
pub struct BorrowActionIri;
impl PartialEq<&str> for BorrowActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BORROW_ACTION_IRI_HTTP || *other == BORROW_ACTION_IRI_HTTPS
	}
}
impl PartialEq<BorrowActionIri> for &str {
	fn eq(&self, other: &BorrowActionIri) -> bool {
		*self == BORROW_ACTION_IRI_HTTP || *self == BORROW_ACTION_IRI_HTTPS
	}
}
pub struct BorrowActionIriOrLabel;
impl PartialEq<&str> for BorrowActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BorrowActionIri || *other == BORROW_ACTION_LABEL
	}
}
impl PartialEq<BorrowActionIriOrLabel> for &str {
	fn eq(&self, other: &BorrowActionIriOrLabel) -> bool {
		*self == BorrowActionIri || *self == BORROW_ACTION_LABEL
	}
}
