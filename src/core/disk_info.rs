use serde::{Serialize, Deserialize}; // For parsing JSON later

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemDiskInfo {
    pub disks: Vec<PhysicalDisk>,
    pub lvm_volume_groups: Vec<LvmVolumeGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhysicalDisk {
    pub path: String,                    // e.g., /dev/sda
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub size_bytes: u64,
    pub rota: bool, // Rotational (HDD) vs SSD
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Partition {
    pub path: String,                    // e.g., /dev/sda1
    pub part_type_guid: Option<String>,  // GPT Partition Type GUID
    pub part_label: Option<String>,      // GPT Partition Label
    pub part_uuid: Option<String>,       // GPT Partition UUID
    pub part_flags: Option<String>,      // e.g., boot, esp
    pub fs_type: Option<String>,         // e.g., ext4, ntfs, crypto_LUKS, LVM2_member
    pub fs_uuid: Option<String>,         // Filesystem UUID
    pub fs_label: Option<String>,        // Filesystem Label
    pub size_bytes: u64,
    pub mount_point: Option<String>,
    pub content: Option<PartitionContent>, // What is *inside* the partition?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartitionContent {
    FileSystem,
    LuksContainer {
        uuid: String,
        mapped_name: Option<String>,
        mapped_content: Option<Box<MappedContent>>,
    },
    LvmPhysicalVolume {
        pv_uuid: String,
        vg_name: Option<String>,
    },
    VeraCryptContainer { // Placeholder
        is_mounted: bool,
        mount_path: Option<String>,
    },
    Unknown,
    Swap,
}

// Default for PartitionContent if needed, though Option<PartitionContent> in Partition handles None case.
// impl Default for PartitionContent {
//     fn default() -> Self {
//         PartitionContent::Unknown
//     }
// }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MappedContent { // Content of an opened LUKS container
    LvmPhysicalVolume(LvmPhysicalVolumeData),
    FileSystem {
        fs_type: Option<String>,
        fs_uuid: Option<String>,
        fs_label: Option<String>,
        mount_point: Option<String>,
    },
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LvmPhysicalVolumeData { // Data for a PV, whether directly on partition or in LUKS
    pub path: String, // Path to the PV (e.g., /dev/sda2 or /dev/mapper/luks-xyz)
    pub pv_uuid: String,
    pub vg_name: Option<String>,
    pub size_bytes: u64,
    pub free_bytes: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LvmVolumeGroup {
    pub name: String,
    pub uuid: String,
    pub size_bytes: u64,
    pub free_bytes: u64,
    pub physical_volumes: Vec<String>, // Paths to the PVs (e.g. /dev/sda2, /dev/mapper/luks-on-sdb1)
    pub logical_volumes: Vec<LvmLogicalVolume>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LvmLogicalVolume {
    pub name: String,
    pub path: String,                    // e.g., /dev/vg_name/lv_name
    pub uuid: String,
    pub size_bytes: u64,
    pub fs_type: Option<String>,
    pub fs_uuid: Option<String>,
    pub fs_label: Option<String>,
    pub mount_point: Option<String>,
}

// Function to create dummy disk information for UI development
pub fn create_dummy_system_disk_info() -> SystemDiskInfo {
    let mut disks = Vec::new();
    let mut lvm_vgs = Vec::new();

    // --- Disk 1: /dev/sda (SSD with OS, LUKS on LVM) ---
    let mut sda_partitions = Vec::new();
    // sda1: EFI
    sda_partitions.push(Partition {
        path: "/dev/sda1".to_string(),
        part_type_guid: Some("C12A7328-F81F-11D2-BA4B-00A0C93EC93B".to_string()), // EFI System Partition
        part_label: Some("EFI System Partition".to_string()),
        fs_type: Some("vfat".to_string()),
        fs_uuid: Some("A1B2-C3D4".to_string()),
        size_bytes: 512 * 1024 * 1024, // 512 MiB
        mount_point: Some("/boot/efi".to_string()),
        content: Some(PartitionContent::FileSystem),
        ..Default::default()
    });
    // sda2: LUKS Container for LVM
    let sda2_luks_content = PartitionContent::LuksContainer {
        uuid: "luks-uuid-sda2".to_string(),
        mapped_name: Some("cr_lvm".to_string()), // crypt_lvm or similar
        mapped_content: Some(Box::new(MappedContent::LvmPhysicalVolume(
            LvmPhysicalVolumeData {
                path: "/dev/mapper/cr_lvm".to_string(),
                pv_uuid: "lvm-pv-uuid-on-cr_lvm".to_string(),
                vg_name: Some("vg_system".to_string()),
                size_bytes: 249_000_000_000 - (512 * 1024 * 1024), // Approx 249GB - EFI
                free_bytes: 10_000_000_000, // 10 GB free in PV (example)
            }
        ))),
    };
    sda_partitions.push(Partition {
        path: "/dev/sda2".to_string(),
        size_bytes: 249_000_000_000 - (512 * 1024 * 1024), // Approx 249GB
        content: Some(sda2_luks_content),
        fs_type: Some("crypto_LUKS".to_string()),
        ..Default::default()
    });

    disks.push(PhysicalDisk {
        path: "/dev/sda".to_string(),
        model: Some("Samsung SSD 970 EVO".to_string()),
        vendor: Some("Samsung".to_string()),
        size_bytes: 250 * 1024 * 1024 * 1024, // 250 GB
        rota: false, // SSD
        partitions: sda_partitions,
    });

    // LVM VG "vg_system" on sda2 (inside LUKS)
    let mut vg_system_lvs = Vec::new();
    vg_system_lvs.push(LvmLogicalVolume {
        name: "lv_root".to_string(),
        path: "/dev/vg_system/lv_root".to_string(),
        uuid: "lv-uuid-root".to_string(),
        size_bytes: 100 * 1024 * 1024 * 1024, // 100 GB
        fs_type: Some("ext4".to_string()),
        mount_point: Some("/".to_string()),
        ..Default::default()
    });
    vg_system_lvs.push(LvmLogicalVolume {
        name: "lv_home".to_string(),
        path: "/dev/vg_system/lv_home".to_string(),
        uuid: "lv-uuid-home".to_string(),
        size_bytes: 130 * 1024 * 1024 * 1024, // 130 GB
        fs_type: Some("ext4".to_string()),
        mount_point: Some("/home".to_string()),
        ..Default::default()
    });
    lvm_vgs.push(LvmVolumeGroup {
        name: "vg_system".to_string(),
        uuid: "vg-uuid-system".to_string(),
        size_bytes: 240_000_000_000, // Approx 240 GB
        free_bytes: 10_000_000_000,  // 10 GB free in VG
        physical_volumes: vec!["/dev/mapper/cr_lvm".to_string()],
        logical_volumes: vg_system_lvs,
    });

    // --- Disk 2: /dev/sdb (HDD with NTFS and some free space) ---
    let mut sdb_partitions = Vec::new();
    // sdb1: NTFS (Windows Data?)
    sdb_partitions.push(Partition {
        path: "/dev/sdb1".to_string(),
        fs_type: Some("ntfs".to_string()),
        fs_label: Some("WindowsData".to_string()),
        size_bytes: 500 * 1024 * 1024 * 1024, // 500 GB
        mount_point: None, // Not mounted in this example
        content: Some(PartitionContent::FileSystem),
        ..Default::default()
    });
    // sdb2: Unallocated/Unknown (for new Lunitool install?)
    sdb_partitions.push(Partition {
        path: "/dev/sdb2".to_string(), // This might not exist as a path yet if unallocated
        size_bytes: 500 * 1024 * 1024 * 1024, // 500 GB Free Space (example)
        content: Some(PartitionContent::Unknown), // Represents free, unformatted space
        ..Default::default()
    });

    disks.push(PhysicalDisk {
        path: "/dev/sdb".to_string(),
        model: Some("WD Blue HDD".to_string()),
        vendor: Some("Western Digital".to_string()),
        size_bytes: 1000 * 1024 * 1024 * 1024, // 1 TB
        rota: true, // HDD
        partitions: sdb_partitions,
    });

    // --- Disk 3: /dev/nvme0n1 (NVMe with just Swap and LVM PV for another VG) ---
    let mut nvme_partitions = Vec::new();
    nvme_partitions.push(Partition {
        path: "/dev/nvme0n1p1".to_string(),
        fs_type: Some("linux-swap".to_string()),
        size_bytes: 16 * 1024 * 1024 * 1024, // 16 GB Swap
        content: Some(PartitionContent::Swap),
        ..Default::default()
    });
    let nvme_lvm_pv_content = PartitionContent::LvmPhysicalVolume {
        pv_uuid: "lvm-pv-uuid-on-nvme".to_string(),
        vg_name: Some("vg_data".to_string()),
    };
    nvme_partitions.push(Partition {
        path: "/dev/nvme0n1p2".to_string(),
        fs_type: Some("LVM2_member".to_string()),
        size_bytes: 480 * 1024 * 1024 * 1024, // 480 GB for LVM
        content: Some(nvme_lvm_pv_content),
        ..Default::default()
    });
    disks.push(PhysicalDisk {
        path: "/dev/nvme0n1".to_string(),
        model: Some("Kingston NVMe".to_string()),
        vendor: Some("Kingston".to_string()),
        size_bytes: 500 * 1024 * 1024 * 1024, // ~500GB NVMe
        rota: false,
        partitions: nvme_partitions,
    });

    // LVM VG "vg_data" on nvme0n1p2
    let mut vg_data_lvs = Vec::new();
    vg_data_lvs.push(LvmLogicalVolume {
        name: "lv_games".to_string(),
        path: "/dev/vg_data/lv_games".to_string(),
        uuid: "lv-uuid-games".to_string(),
        size_bytes: 480 * 1024 * 1024 * 1024, // 480 GB
        fs_type: Some("btrfs".to_string()),
        mount_point: Some("/mnt/games".to_string()),
        ..Default::default()
    });
    lvm_vgs.push(LvmVolumeGroup {
        name: "vg_data".to_string(),
        uuid: "vg-uuid-data".to_string(),
        size_bytes: 480 * 1024 * 1024 * 1024, // Approx 480 GB
        free_bytes: 0,  
        physical_volumes: vec!["/dev/nvme0n1p2".to_string()],
        logical_volumes: vg_data_lvs,
    });

    SystemDiskInfo {
        disks,
        lvm_volume_groups: lvm_vgs,
    }
} 