use crate::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;
use crate::error::{HeaderTypeReason, OldPciError, OldPciResult};

#[derive(Debug)]
pub enum SingleFunctionDevice {
    General(GeneralHeader),
    PciToPciBride(PciToPciBridgeHeader),
}

impl SingleFunctionDevice {
    pub fn expect_general(self) -> OldPciResult<GeneralHeader> {
        if let Self::General(general) = self {
            Ok(general)
        } else {
            Err(OldPciError::InvalidHeaderType(
                HeaderTypeReason::NotGeneralHeader,
            ))
        }
    }
}
