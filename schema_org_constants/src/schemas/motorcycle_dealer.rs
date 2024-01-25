/// <https://schema.org/MotorcycleDealer>
pub const MOTORCYCLE_DEALER_IRI_HTTP: &str = "http://schema.org/MotorcycleDealer";
/// <https://schema.org/MotorcycleDealer>
pub const MOTORCYCLE_DEALER_IRI_HTTPS: &str = "https://schema.org/MotorcycleDealer";
/// <https://schema.org/MotorcycleDealer>
pub const MOTORCYCLE_DEALER_LABEL: &str = "MotorcycleDealer";
pub struct MotorcycleDealerIri;
impl PartialEq<&str> for MotorcycleDealerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOTORCYCLE_DEALER_IRI_HTTP || *other == MOTORCYCLE_DEALER_IRI_HTTPS
	}
}
impl PartialEq<MotorcycleDealerIri> for &str {
	fn eq(&self, other: &MotorcycleDealerIri) -> bool {
		*self == MOTORCYCLE_DEALER_IRI_HTTP || *self == MOTORCYCLE_DEALER_IRI_HTTPS
	}
}
pub struct MotorcycleDealerIriOrLabel;
impl PartialEq<&str> for MotorcycleDealerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MotorcycleDealerIri || *other == MOTORCYCLE_DEALER_LABEL
	}
}
impl PartialEq<MotorcycleDealerIriOrLabel> for &str {
	fn eq(&self, other: &MotorcycleDealerIriOrLabel) -> bool {
		*self == MotorcycleDealerIri || *self == MOTORCYCLE_DEALER_LABEL
	}
}
