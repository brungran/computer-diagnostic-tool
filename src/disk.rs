use sysinfo::Disks;

pub fn info(){
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let path_str = disk.mount_point().to_string_lossy();
        let drive_letter = path_str.trim_end_matches('\\');
        println!("Disk\t\t\tType\tSpace (GB)\tUsage (MBs Wrote/MBs Read)");
        println!("{:?} ({:?}):\t{:?}\t{:?}/{:?}\t\t{:?}/{:?}",
            disk.name(),
            drive_letter,
            disk.kind(),
            disk.available_space()/1000000000,
            disk.total_space()/1000000000,
            disk.usage().total_written_bytes/1000000,
            disk.usage().total_read_bytes/1000000
        );
    }
}