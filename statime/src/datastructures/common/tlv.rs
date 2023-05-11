use arrayvec::ArrayVec;

use crate::datastructures::{WireFormat, WireFormatError};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TLV {
    pub tlv_type: TlvType,
    pub length: u16,

    // TODO: Determine the best max value
    pub value: ArrayVec<u8, 128>,
}

impl WireFormat for TLV {
    fn wire_size(&self) -> usize {
        (4 + self.length).into()
    }

    fn serialize(&self, buffer: &mut [u8]) -> Result<(), WireFormatError> {
        buffer[0..1].copy_from_slice(&self.tlv_type.to_primitive().to_be_bytes());
        buffer[2..3].copy_from_slice(&self.length.to_be_bytes());
        buffer[4..(4 + self.length).into()].copy_from_slice(&self.value.as_slice());

        Ok(())
    }

    fn deserialize(buffer: &[u8]) -> Result<Self, WireFormatError> {
        if buffer.len() < 5 {
            return Err(WireFormatError::BufferTooShort);
        }

        // Parse length
        let length_bytes: Result<[u8; 2], _> = buffer[2..3].try_into();
        if length_bytes.is_err() {
            return Err(WireFormatError::BufferTooShort);
        }
        let length = u16::from_be_bytes(length_bytes.unwrap());

        // Parse TLV content / value
        if buffer.len() < (5 + length) as usize {
            return Err(WireFormatError::BufferTooShort);
        }

        let mut vec = ArrayVec::<u8, 128>::new();
        for byte in &buffer[4..(4 + length).into()] {
            if !vec.try_push(*byte).is_ok() {
                return Err(WireFormatError::CapacityError);
            }
        }

        // Parse TLV type
        let type_bytes = buffer[0..1].try_into();
        if type_bytes.is_err() {
            return Err(WireFormatError::BufferTooShort);
        }

        Ok(Self {
            tlv_type: TlvType::from_primitive(u16::from_be_bytes(type_bytes.unwrap())),
            length,
            value: vec,
        })
    }
}

/// See 14.1.1 / Table 52
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlvType {
    Reserved,
    #[default]
    Management,
    ManagementErrorStatus,
    OrganizationExtension,
    RequestUnicastTransmission,
    GrantUnicastTransmission,
    CancelUnicastTransmission,
    AcknowledgeCancelUnicastTransmission,
    PathTrace,
    AlternateTimeOffsetIndicator,
    Legacy,
    Experimental,
    OrganizationExtensionPropagate,
    EnhancedAccuracyMetrics,
    OrganizationExtensionDoNotPropagate,
    L1Sync,
    PortCommunicationAvailability,
    ProtocolAddress,
    SlaveRxSyncTimingData,
    SlaveRxSyncComputedData,
    SlaveTxEventTimestamps,
    CumulativeRateRatio,
    Pad,
    Authentication,
}

impl TlvType {
    pub fn to_primitive(&self) -> u16 {
        match self {
            Self::Reserved => 0x0000,
            Self::Management => 0x0001,
            Self::ManagementErrorStatus => 0x0002,
            Self::OrganizationExtension => 0x0003,
            Self::RequestUnicastTransmission => 0x0004,
            Self::GrantUnicastTransmission => 0x0005,
            Self::CancelUnicastTransmission => 0x0006,
            Self::AcknowledgeCancelUnicastTransmission => 0x0007,
            Self::PathTrace => 0x0008,
            Self::AlternateTimeOffsetIndicator => 0x0009,
            Self::Legacy => 0x2000,
            Self::Experimental => 0x2004,
            Self::OrganizationExtensionPropagate => 0x4000,
            Self::EnhancedAccuracyMetrics => 0x4001,
            Self::OrganizationExtensionDoNotPropagate => 0x8000,
            Self::L1Sync => 0x8001,
            Self::PortCommunicationAvailability => 0x8002,
            Self::ProtocolAddress => 0x8003,
            Self::SlaveRxSyncTimingData => 0x8004,
            Self::SlaveRxSyncComputedData => 0x8005,
            Self::SlaveTxEventTimestamps => 0x8006,
            Self::CumulativeRateRatio => 0x8007,
            Self::Pad => 0x8008,
            Self::Authentication => 0x8009,
        }
    }

    pub fn from_primitive(value: u16) -> Self {
        match value {
            0x0000
            | 0x000a..=0x1fff
            | 0x2030..=0x3fff
            | 0x4002..=0x7eff
            | 0x800a..=0xffef
            | 0xfff0..=0xffff => Self::Reserved,
            0x2000..=0x2003 => Self::Legacy,
            0x2004..=0x202f | 0x7f00..=0x7fff => Self::Experimental,
            0x0001 => Self::Management,
            0x0002 => Self::ManagementErrorStatus,
            0x0003 => Self::OrganizationExtension,
            0x0004 => Self::RequestUnicastTransmission,
            0x0005 => Self::GrantUnicastTransmission,
            0x0006 => Self::CancelUnicastTransmission,
            0x0007 => Self::AcknowledgeCancelUnicastTransmission,
            0x0008 => Self::PathTrace,
            0x0009 => Self::AlternateTimeOffsetIndicator,
            0x4000 => Self::OrganizationExtensionPropagate,
            0x4001 => Self::EnhancedAccuracyMetrics,
            0x8000 => Self::OrganizationExtensionDoNotPropagate,
            0x8001 => Self::L1Sync,
            0x8002 => Self::PortCommunicationAvailability,
            0x8003 => Self::ProtocolAddress,
            0x8004 => Self::SlaveRxSyncTimingData,
            0x8005 => Self::SlaveRxSyncComputedData,
            0x8006 => Self::SlaveTxEventTimestamps,
            0x8007 => Self::CumulativeRateRatio,
            0x8008 => Self::Pad,
            0x8009 => Self::Authentication,
        }
    }
}
