// Four basic regions
//  0. Reserved Region
//  1. FAT Region
//  2. Root Directory Region
//  3. File and Directory Data Region

#[repr(transparent)]
#[derive(Debug)]
struct FatBootSector {
    /// Jump instructions to boot code. 
    /// This field has two allowed forms:
    ///     1. jmp[0] = 0xEB, jmp[1] = 0x??, jmpBoot[2] = 0x90
    ///     2. jmp[0] = 0xE9, jmp[1] = 0x??, jmp[2] = 0x??
    ///
    /// This forms a three-byte Intel x86 unconditional branch instruction.
    /// The boot code typically occupies the rest of sector 0 of the volume
    /// following the BPB and possibly other sectors. 
    /// 
    /// jmp[0] = 0xEB is the more frequently used format
    jmp: [i8; 3],

    /// An indication of what system formatted the volume.
    /// Use "MSWIN4.1" for compatability purposes
    oem_system: [u8; 8],
    
    /// Can only take the following values: 512, 1024, 2048, or 4096
    /// For maximum compatability 512 should be used.
    bytes_per_sector: u16,
    
    /// Must be a power of 2 greater than 0 (1, 2, 4, 8, 16, 32, 64, or 128).
    /// Note that a value should never be used that results in a "bytes per cluster"
    /// value (bytes_per_sector * sectors_per_cluster) greater than 32K.
    sectors_per_cluster: i8,
    
    /// Number of reserved sectors in the reserved region of the volume starting
    /// at the first sector of the volume. The field must not be zero. 
    ///
    /// For FAT12 and FAT16 volumes this must be 1. 
    /// For fat 32 volumes, this value is typically 32.
    reserved_sector_count: u16,
    
    /// The count of FAT data structures on the volume. This field should always
    /// contain the value 2 for any FAT volume of any type.
    table_count: i8,
  
    /// For FAT12 and FAT16 volumes, this field contains the count of 32-byte
    /// directory entries in the root directory.
    ///
    /// For FAT32 volumes, this field must be set to 0.
    /// For FAT12 and FAT16 volumes, this value should always specify a count that
    /// when multiplied by 32 results in an even multile of bytes_per_sector.
    /// For maximum compatibility, FAT16 volumes should use the value 512
    ///
    root_entry_count: u16,

    /// This field is the old 16-bit total count of sectors on the volume. This count
    /// includes the count of all sectors in all four regions of the volume. 
    ///
    /// This field can be 0; if it is 0, then sector_count_32 must be non-zero.
    ///
    /// For FAT32 volumes, this field must be 0.
    /// For FAT12 and FAT16 volumes, this filed contains the sector count, and 
    /// sector_count_32 is 0 if the total sector count "fits" (is less than 0x10000)
    sector_count_16: u16,
    
    /// 0xF8 is the standard value for "fixed" (non-removable) media.
    /// For removable media, 0xF0 is frequently used.
    /// 
    /// The legal values for this field are 0xF0, 0xF8, 0xF9, 0xFB, 0xFC, 0xFD,
    /// 0xFE, and 0xFF.
    ///
    /// Whatever value is put in here must also be put in the low byte of the
    /// FAT[0] entry.
    media_type: i8,
    
    /// Sectors per track for interrupt 0x13. This field is only relevant for
    /// media that have a geometry (volume is broken down into tracks by
    /// multiple heads and cylinders) and are visible on interrupt 0x13.
    /// This field contains the "sectors per track" geometry value.
    sectors_per_track: u16,
    
    /// Number of heads for interrupt 0x13. This field is relevant as discussed
    /// earlier for sectors_per_track.
    heads: u16,
    
    /// Count of hidden sectors preceding the partition that contains this FAT
    /// volume. This field is generally only relevant for media visible on
    /// interrupt 0x13. This field should always be zero on media that are not
    /// partitioned. Exactly what value is appropriate is operating system
    /// specific.
    hidden_sector_count: u32,
    
    /// This field is the new 32-bit total count of sectors on the volume.
    /// This count includes the count of all sectors in all four regions of the
    /// volume.
    ///
    /// This field can be 0; if it is 0, then sector_count_16 must be non-zero.
    /// 
    /// For FAT32 volumes, this field must be non-zero.
    /// For FAT12/FAT16 volumes, this field contains the sector count if 
    /// sector_count_16 is 0 (count is greater than or equal to 0x10000)
    sector_count_32: u32,
}

#[repr(transparent)]
#[derive(Debug)]
struct FatExtension {
    /// Int 0x13 drive number.
    drive_number: u8,

    /// Reserved (used by Windows NT). Code that formats FAT volumes should always
    /// set this byte to 0.
    reserved: u8,

    /// Extened boot signature (0x29). This is a signature byte that indicates that
    /// the following three fields in the boot sector are present
    boot_signature: u8,
    
    /// Volume serial number. This field, togetheer with volume_label supports
    /// volume tracking on removable media. These values allow FAT file system
    /// drivers to detect that the wrong disk is inserted in a removable drive.
    /// This ID is usually generated by simply combining the current date and time
    /// int a 32-bit value.
    volume_id: u32,
    
    /// Volume label. This field matches the 11-byte volume label recorded in the
    /// root directory.
    ///
    /// FAT file system drivers should make sure that they update this field when
    /// the volume label file in the root directory has its name changed or created.
    /// The setting for this field when there is no volume label is the string
    /// "NO NAME    ".
    volume_label: [u8; 11],
    
    /// One of the strings "FAT12   ", "FAT16   ", or "FAT  ". 
    file_system_type: [u8; 8],
}

#[repr(transparent)]
#[derive(Debug)]
/// This extension is not found in FAT12/FAT16 volumes
struct Fat32Extension {
    /// This field is the FAT32 32-bit count of sectors occupied by ONE FAT. 
    /// fat_size_16 must be zero
    fat_size_32: u32,

    /// Bits 0-3:
    ///     Zero based number of active FAT. Only valid if mirroring is disabled
    /// Bits 4-6:
    ///     Reserved
    /// Bit 7
    ///     0 means the FAT is mirrored at runtime into all FATs
    ///     1 means only one FAT is active; it is the references in bits 0-3
    /// Bits 8-15
    ///     Reserved
    ext_flags: u16,
    
    /// High byte is major revision number.
    /// Low byte is minor revision number.
    /// This is the version number of the FAT32 volume. This supports the ability
    /// to extend the FAT32 media type in the future without worrying about old
    /// FAT32 drivers mounting the volume.
    ///
    /// Disk utilities should respect this field and not operate on volumes with a
    /// higher major or minor version number than that for which they were designed.
    /// FAT32 file system drivers must check this field and not mount the volume if
    /// it does not contain a version number that was defined at the time the driver
    /// was written.
    fs_version: u16,
    
    /// This is set to the cluster number of thee first cluster of the root
    /// directory, usually 2 but not required to be 2.
    ///
    /// Disk utilities that change the location of the root directory should make
    /// every effort to place the first cluster of the root directory in the first
    /// non-bad cluster on the drive (i.e. in cluster 2 unless it's marked bad).
    /// This is specified so that disk repair utilities can easily find the root
    /// directory if this field accidently gets zeroed.
    root_cluster: u32,
    
    /// Sector number of FSINFO structure in the reserved area of the FAT32 volume.
    /// Usually 1.
    ///
    /// There will be a copy of the FSINFO structure in BackupBoot, but only
    /// the copy pointed to by this field will be kept up to date (i.e., both the 
    /// primary and backup boot record will point to the same FSINFO sector)
    fs_info: u16,
    
    /// If non-zero, indicates the sector number in the reserved area of the 
    /// volume of a copy of the boot record. 
    /// 
    /// No value other than 6 is recommended.
    bk_boot_sec: u16,
    
    /// Reserved for future expansion. Code that formats FAT32 volumes should always
    /// set all of thee bytes of this field to 0.
    reserved: [u8; 12],
    
    fat_extension: Fat_Extension,
}

#[repr(transparent)]
#[derive(Debug)]
struct Fat32BootSector {
    fat_boot_sector: FatBootSector,
    extension: Fat32Extension,
}

#[repr(transparent)]
#[derive(Debug)]
struct FatBootSector {
    fat_boot_sector: FatBootSector,
    extension: FatExtension,
}

#[repr(transparent)]
#[derive(Debug)]
enum FileAttributes {
    /// Indicates that writes to the file should fail
    ATTR_READ_ONLY,

    /// Indicates that normal directory listings should not show this file
    ATTR_HIDDEN,

    /// Indicates that this is an operating system file
    ATTR_SYSTEM,

    /// There should only be one "file" on the volume that has this attribute set
    /// and the file must be in the root directory. This name of this file is
    /// actually the label for the volume. frst_clus_hi and frst_clus_lo must always
    /// be 0 for the volume label (no data clusters are allocated to the 
    /// volume label file).
    ATTR_VOLUME_ID,

    /// Indicates that this file is actually a container for other files
    ATTR_DIRECTORY,

    /// This archive supports backup utilities. This bit is set by the FAT file
    /// system driver when a file is created, renamed, or written to. Backup
    /// utilities may use this attribute to indicate which files on the volume have
    /// been modified since the last time that a backup was performed.
    ATTR_ARCHIVE,
}

#[repr(transparent)]
#[derive(Debug)]
struct Fat32DirectoryEntry {
    /// Short name
    ///
    /// If name[0] == 0xE5, then the directory entry is free (there is no file
    /// or directory name in this entry).
    ///
    /// If name[0] == 0x00, then the directory is free (same as for 0xE5), and
    /// there are no allocated directory entries after this one (all of name[0]
    /// bytes in all of the entries after this one are also set to 0).
    ///
    /// The special 0 value indicates to FAT file system driveer code that the 
    /// rest of the entries in this directory do not need to be examined
    /// because they are all free.
    ///
    /// If name[0] == 0x05, then the actual file name character for this byte is
    /// 0xE5. 0xE5 is actually a valid KANJI lead byte value for the character
    /// set used in Japan. The special 0x05 value is used so that this special file
    /// name case for japan can be handled properly and not cause FAT file system
    /// code to think that the entry is free
    ///
    /// The following characters are not legal in any bytes of name:
    ///     - Values less than 0x20 except for the special case 0x05
    ///     - 0x22, 0x2A, 0x2B, 0x2C, 0x2E, 0x2F, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E
    ///     0x3F, 0x5B, 0x5C, 0x5D
    ///
    /// Each name is required to be unique.
    /// Lowercase characters are not allowed in name
    name: [u8; 11],
    
    /// File attributes
    attr: u8,
    
    /// Reserved for use by Windows NT. Set value to 0 when a file is created
    /// and never modify or look at it after that
    nt_res: u8,
    
    /// Milisecond stamp at file creation time. This field actually 
    /// contains a count of tenths of a second. The granularity of the seconds part
    /// is 2 seconds so this field is a count of tenths of a second and its valid
    /// range is 0-199 inclusive
    crt_time_tenth: u8,
    
    /// Time file was created
    crt_time: Time,
    
    /// Date file was created
    crt_date: Date,
    
    /// Last access date. Note that there is no last access time, only a date.
    /// This is the date of the last read or write. In the case of a write, this
    /// should be set to the same date as wrt_date
    dir_last_acc_date: Date,
    
    /// High word of this entry's first cluster number (always 0 for a 
    /// FAT12 or FAT16 volume)
    fst_clus_hi: u16,
    
    /// Time of last write. Note that file creation is considered a write
    wrt_time: Time,
    
    /// Date of last write. Note that file creation is considered a write
    wrt_date: Time,
    
    /// Low word of this entry's first cluster number
    fst_clus_lo: u16,
    
    /// 32-bit DWORD holding this file's size in bytes
    file_size: u32
}

#[repr(transparent)]
#[derive(Debug)]
struct LongFileNameEntry {
    /// The order of this entry in the sequence of long dir entries associated
    /// with the short dir entry at the end of the long dir set
    /// 
    /// If masked with 0x40 (LAST_LONG_ENTRY), this indicates the entry is the last
    /// long dir entry in a set of long dir entries. All valid set of long dir
    /// entries must begin with an entry having this mask.
    entry_number: u8,

    /// Characters 1-5 of the long-name sub-component in this dir entry.
    name1: [u16; 5],
    
    /// Must be ATTR_LONG_NAME
    long_file_attribute: u8,
    
    /// If zero, indicates a directory entry that is a sub-component of a long name.
    /// 
    /// Non-zero implies other dirent types
    long_entry_type: u8,
    
    /// Checksum of name in the short dir entry at the end of the long dir set
    checksum: u8,
    
    /// Characters 6-11 of the long-name sub-component in this dir entry
    name2: [u16; 12],
    
    /// Must be 0.
    zero: [u16],
    
    /// Characters 12-13 of the long-name sub-component in this dir entry
    name3: [u16, 2],

}

// Date and time formats. Many FAT file systems do not support Date/Time other than 
// write_time and write_date. For this reason, crt_time_mil, crt_time, crt_date, and
// lst_acc_date are actually optional fields.
//
// If the other date and time fields are not supported, they should be set to 0 on
// file create and ignored on other file operations.

#[repr(transparent)]
#[derive(Debug)]
/// A 16-bit field that is basically a date relative to the MS-DOS epoch of
/// 01/01/1980
struct Date {
     raw: u16;
}
impl Date {
    /// Gets the day of month
    pub fn day(&self) -> u8 {
       (self | 0x001F)
    }
    
    /// Gets the month of the year
    pub fn month(&self) -> u8 {
        ((self | 0x01E0) >> 5)
    }
    
    /// Gets the year created
    pub fn year(&self) -> u8 {
        ((self | 0xFE00) >> 9) + 1980
    }
}

#[repr(transparent)]
#[derive(Debug)]
/// A 16-bit field that has a granularity of 2 seconds.
struct Time {
    pub fn seconds(&self) -> u8 {
        (self | 0x001F)
    }
    pub fn minutes(&self) -> u8 {
        (self | 0x07E0) >> 5
    }
    pub fn hours(&self) -> u8 {
        (self | 0xF800) >> 11
    }
}
