use sysinfo::{Disk, DiskKind, Disks};


#[derive(serde::Serialize)]
pub struct DriveInfo {
   pub name: String,
    pub mount_point: String,
    pub kind: String,
}

impl DriveInfo {
    pub fn from(disk: &Disk) -> DriveInfo {
        let mount_point = disk
            .mount_point()
            .to_owned()
            .into_os_string()
            .into_string()
            .unwrap();

        DriveInfo {
            name: disk.name().to_str().unwrap().to_string(),
            mount_point,
            kind: disk.kind().to_string(),
        }
    }
}

#[tauri::command]
pub fn get_drive_info() -> Vec<DriveInfo> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    let list_of_drive_info = disks.iter().map(|disk_item| DriveInfo::from(disk_item));
    list_of_drive_info.collect()
}


