use std::fs::FileType;

#[derive(Debug, Default)]
pub struct Reader;
impl Reader {
    pub(super) fn read_file(&self, path: &String) -> (FileType, String) {
        todo!("read file")
    }

    pub(super) fn read_from_path(&self, path: &String) -> Vec<(FileType, String)> {
        Vec::new()
        //leggi le cose nel path e quelle nelle prime N cartelle tramite single_read_from_path
    }

    fn single_read_from_path(&self, path: &String) -> Vec<(FileType, String)> {
        Vec::new()
        //leggi solo le cose in questo path
    }
}
