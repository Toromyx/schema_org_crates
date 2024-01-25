/// <https://schema.org/doesNotShip>
pub const DOES_NOT_SHIP_PROPERTY_IRI_HTTP: &str = "http://schema.org/doesNotShip";
/// <https://schema.org/doesNotShip>
pub const DOES_NOT_SHIP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/doesNotShip";
/// <https://schema.org/doesNotShip>
pub const DOES_NOT_SHIP_PROPERTY_LABEL: &str = "doesNotShip";
pub struct DoesNotShipPropertyIri;
impl PartialEq<&str> for DoesNotShipPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOES_NOT_SHIP_PROPERTY_IRI_HTTP || *other == DOES_NOT_SHIP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DoesNotShipPropertyIri> for &str {
	fn eq(&self, other: &DoesNotShipPropertyIri) -> bool {
		*self == DOES_NOT_SHIP_PROPERTY_IRI_HTTP || *self == DOES_NOT_SHIP_PROPERTY_IRI_HTTPS
	}
}
pub struct DoesNotShipPropertyIriOrLabel;
impl PartialEq<&str> for DoesNotShipPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoesNotShipPropertyIri || *other == DOES_NOT_SHIP_PROPERTY_LABEL
	}
}
impl PartialEq<DoesNotShipPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DoesNotShipPropertyIriOrLabel) -> bool {
		*self == DoesNotShipPropertyIri || *self == DOES_NOT_SHIP_PROPERTY_LABEL
	}
}
