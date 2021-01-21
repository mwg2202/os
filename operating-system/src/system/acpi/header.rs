pub struct ACPISDTHeader {
  Signature: [u8, 4],
  Length: u32,
  Revision: u8,
  Checksum: u8,
  OEMID: [u8, 6],
  OEMTableID: [u8, 8],
  OEMRevision: u32,
  CreatorID: u32,
  CreatorRevision: u32,
}
