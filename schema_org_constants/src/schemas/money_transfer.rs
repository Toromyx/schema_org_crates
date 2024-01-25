/// <https://schema.org/MoneyTransfer>
pub const MONEY_TRANSFER_IRI_HTTP: &str = "http://schema.org/MoneyTransfer";
/// <https://schema.org/MoneyTransfer>
pub const MONEY_TRANSFER_IRI_HTTPS: &str = "https://schema.org/MoneyTransfer";
/// <https://schema.org/MoneyTransfer>
pub const MONEY_TRANSFER_LABEL: &str = "MoneyTransfer";
pub struct MoneyTransferIri;
impl PartialEq<&str> for MoneyTransferIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONEY_TRANSFER_IRI_HTTP || *other == MONEY_TRANSFER_IRI_HTTPS
	}
}
impl PartialEq<MoneyTransferIri> for &str {
	fn eq(&self, other: &MoneyTransferIri) -> bool {
		*self == MONEY_TRANSFER_IRI_HTTP || *self == MONEY_TRANSFER_IRI_HTTPS
	}
}
pub struct MoneyTransferIriOrLabel;
impl PartialEq<&str> for MoneyTransferIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MoneyTransferIri || *other == MONEY_TRANSFER_LABEL
	}
}
impl PartialEq<MoneyTransferIriOrLabel> for &str {
	fn eq(&self, other: &MoneyTransferIriOrLabel) -> bool {
		*self == MoneyTransferIri || *self == MONEY_TRANSFER_LABEL
	}
}
