/// <https://schema.org/TransferAction>
pub const TRANSFER_ACTION_IRI_HTTP: &str = "http://schema.org/TransferAction";
/// <https://schema.org/TransferAction>
pub const TRANSFER_ACTION_IRI_HTTPS: &str = "https://schema.org/TransferAction";
/// <https://schema.org/TransferAction>
pub const TRANSFER_ACTION_LABEL: &str = "TransferAction";
pub struct TransferActionIri;
impl PartialEq<&str> for TransferActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSFER_ACTION_IRI_HTTP || *other == TRANSFER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TransferActionIri> for &str {
	fn eq(&self, other: &TransferActionIri) -> bool {
		*self == TRANSFER_ACTION_IRI_HTTP || *self == TRANSFER_ACTION_IRI_HTTPS
	}
}
pub struct TransferActionIriOrLabel;
impl PartialEq<&str> for TransferActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransferActionIri || *other == TRANSFER_ACTION_LABEL
	}
}
impl PartialEq<TransferActionIriOrLabel> for &str {
	fn eq(&self, other: &TransferActionIriOrLabel) -> bool {
		*self == TransferActionIri || *self == TRANSFER_ACTION_LABEL
	}
}
