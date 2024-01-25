/// <https://schema.org/AutoDealer>
pub const AUTO_DEALER_IRI_HTTP: &str = "http://schema.org/AutoDealer";
/// <https://schema.org/AutoDealer>
pub const AUTO_DEALER_IRI_HTTPS: &str = "https://schema.org/AutoDealer";
/// <https://schema.org/AutoDealer>
pub const AUTO_DEALER_LABEL: &str = "AutoDealer";
pub struct AutoDealerIri;
impl PartialEq<&str> for AutoDealerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_DEALER_IRI_HTTP || *other == AUTO_DEALER_IRI_HTTPS
	}
}
impl PartialEq<AutoDealerIri> for &str {
	fn eq(&self, other: &AutoDealerIri) -> bool {
		*self == AUTO_DEALER_IRI_HTTP || *self == AUTO_DEALER_IRI_HTTPS
	}
}
pub struct AutoDealerIriOrLabel;
impl PartialEq<&str> for AutoDealerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoDealerIri || *other == AUTO_DEALER_LABEL
	}
}
impl PartialEq<AutoDealerIriOrLabel> for &str {
	fn eq(&self, other: &AutoDealerIriOrLabel) -> bool {
		*self == AutoDealerIri || *self == AUTO_DEALER_LABEL
	}
}
