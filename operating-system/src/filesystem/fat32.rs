// Four basic regions
//  0. Reserved Region
//  1. FAT Region
//  2. Root Directory Region
//  3. File and Directory Data Region

#[repr(transparent)]
struct FatBootSector {
    jmp: [i8; 3],
    oem_system: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: i8,
    reserved_sector_count: u16,
    table_count: i8,
    root_entry_count: u16,
    sector_count: u16,
    media_type: i8,
    sectors_if_fat_16: u16, // 0 for fat32
    sectors_per_track: u16,
    heads: u16,
    hidden_sector_count: u32,
    sector_count_32: u32,
}

#[repr(transparent)]
struct FatExtension {
    drive_number: u8,
    reserved: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    file_system_type: [u8; 8],
}

#[repr(transparent)]
struct Fat32Extension {
    sectors_if_fat_32: u32,
    ext_flags: u16,
    fs_version: u16,
    root_cluster: u32,
    fs_info: u16,
    bk_boot_sec: u16,
    reserved: [u8; 12],
    fat_extension: Fat_Extension,
}

#[repr(transparent)]
Fat32BootSector {
    fat_boot_sector: FatBootSector,
    extension: Fat32Extension,
}

#[repr(transparent)]
FatBootSector {
    fat_boot_sector: FatBootSector,
    extension: FatExtension,
}
