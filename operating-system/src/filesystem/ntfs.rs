#[repr(transparent)]
struct ntfs_boot_sector {
    jmp: [i8; 3],
    oem_system: [u8; 8], // Char type
    bytes_per_sector: u16,
    sectors_per_cluster: i8,
    reserved_sector_count: u16,
    table_count: i8,
    root_entry_count: u16,
    sector_count: u16,
    media_type: i8,
    sectors_per_table: u16,
    sectors_per_track: u16,
    heads: u16,
    hidden_sector_count: u32,
    sector_count_32: u32,
    reserved: u32,
    sector_count_64: u64,
}

#[repr(transparent)]
struct ntfs_specific_header {
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
struct master_file_table {
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