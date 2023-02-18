use uefi::{CStr16, Handle};
use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode, RegularFile};
use uefi::table::{Boot, SystemTable};
use common::kib;


/// SimpleFileSystemについてはUEFI.mdを参照
pub(crate) fn open_root_dir(image_handle: Handle, system_table: &SystemTable<Boot>) -> uefi::Result<Directory> {

    return system_table
        .boot_services()
        .get_image_file_system(image_handle)
        .map(|mut sfs| sfs.open_volume())?
}
pub(crate) fn open_file(mut dir: Directory) -> Result<FileHandle, uefi::Error> {
    const FILE_NAME: &str = "mem_map";
    // CStr16はすべての文字を16bitで表します。
    // ファイル名が7文字なのに対し、配列長が8なのは、十分な配列の長さを確保する必要があると
    // ドキュメントに記載されていたためです。
    let mut buff= [0u16; 8];
    let file_name_c_str = CStr16::from_str_with_buf(FILE_NAME, &mut buff).unwrap();
    dir.open(file_name_c_str, FileMode::CreateReadWrite, FileAttribute::empty())
}


pub(crate) fn save_memory_map(mut file: RegularFile, system_table: &mut SystemTable<Boot>) -> uefi::Result {
    let header = b"Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n";
    file.write(header).unwrap();

    // 余裕を持たせるために16KiB
    const MEMORY_MAP_BUFF_SIZE: usize = kib!(16);
    let mut buff = [0u8; MEMORY_MAP_BUFF_SIZE];
    let (_, iter) = system_table
        .boot_services()
        .memory_map(&mut buff)
        .unwrap();

    for memory_descriptor in iter.into_iter(){
        let mut phys_start_buff = memory_descriptor.phys_start.to_be_bytes();
        let mut memory_type_buff =  memory_descriptor.ty.0.to_be_bytes();
        let mut page_count_buff = memory_descriptor.page_count.to_be_bytes();
        file.write(&mut page_count_buff).unwrap();
        file.write(&mut memory_type_buff).unwrap();
        file.write(&mut phys_start_buff).unwrap();
    }
    file.flush()
}


