#[repr(transparent)]
#[derive(Debug)]
struct NtfsBootSector {
    fat_boot_sector: FatBootSector,
    reserved: u32,
    sector_count_64: u64,
}

#[repr(transparent)]
#[derive(Debug)]
struct NtfsSpecificHeader {
    master_file_table_cluster: u64,
    master_file_table_mirror_cluster: u64,
    clusters_per_record: i8,
    reserved: [i8; 3],
    clusters_per_index_buffer: i8,
    reserved: [i8; 3],
    serial_number: u64,
    checksum: u32,
}

#[repr(transparent)]
#[derive(Debug)]
struct MasterFileTable {
    record_type: [u8; 4], // Char type
    update_sequence_offset: u16,
    update_sequence_length: u16,
    log_file_sequence_number: u64,
    record_sequence_number: u16,
    hard_link_cound: u16,
    attributes_offset: u16,
    flags: u16,
    bytes_in_use: u32,
    bytes_allocated: u32,
    parent_record_number: u64,
    next_attribute_index: u32,
    reserved: u32,
    record_number: u64,
}
