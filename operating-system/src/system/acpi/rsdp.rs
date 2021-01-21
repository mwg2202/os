struct RSDPDescriptor {
 Signature: [u8, 8],
 Checksum: u8,
 OEMID: [u8, 6],
 Revision: u8,
 RsdtAddress: u32,
}

struct RSDPDescriptor20 {
 RSDPDescriptor firstPart,
 
 Length: u32,
 XsdtAddress: u64,
 ExtendedChecksum: u8,
 reserved: [u8, 3],
}

