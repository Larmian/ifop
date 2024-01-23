//! - You can use this crate to copy|move|delete|rename file or folder like Explorer that.
//! - When you move a file to another folder, then you can undo or redo in explorer.
//! - Show progress window if this is big file.
//! - The `--flags` options Refer [`IFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperation)::[`SetOperationFlags`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setoperationflags)
//! 
//! ## Installation
//! 
//! ```console
//! $ cargo add ifop
//! ```
//! 
//! 
//! ## Usage: ifop.exe <COMMAND>
//! 
//! ```console
//! Commands:
//!   copy    Copy files from --src to --dest
//!   delete  Delete files from --target
//!   rename  Rename file from --src to --dest
//!   move    Move files from --src to --dest
//!   help    Print this message or the help of the given subcommand(s)
//! 
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
//! 
//! ## Crate Usage
//! 
//! - `Copy` single file
//! ```rust
//! // No flags
//! ifop::copy_file("c:\\src\\file.text", "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::copy_file("c:\\src\\file.text", "c:\\dest", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Copy` multiple files
//! ```rust
//! // No flags
//! ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Move` single file
//! ```rust
//! // No flags
//! ifop::move_file("c:\\src\\file.text", "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::move_file("c:\\src\\file.text", "c:\\dest", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Move` multiple files
//! ```rust
//! // No flags
//! ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Rename` single file
//! ```rust
//! // No flags
//! ifop::rename_file("c:\\src\\folder1", "folder2", None).unwrap();
//! 
//! // With flags
//! ifop::rename_file("c:\\src\\folder1", "folder2", Some(
    //! windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    //! windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Rename` multiple files
//! ```rust
//! // No flags
//! ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt", None).unwrap();
//! 
//! // With flags
//! ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Delete` single file
//! ```rust
//! // No flags
//! ifop::delete_file("c:\\src\\folder1", None).unwrap();
//! // With flags
//! ifop::delete_file("c:\\src\\folder1", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - `Delete` multiple files
//! ```rust
//! // No flags
//! ifop::delete_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], None).unwrap();
//! 
//! // With flags
//! ifop::delete_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - Create file
//! ```rust
//! // No flags
//! ifop::create_file("c:\\", "file.txt", None).unwrap();
//! 
//! // With flags
//! ifop::create_file("c:\\", "file.txt", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! - Create folder
//! ```rust
//! // No flags
//! ifop::create_folder("c:\\", "folder", None).unwrap();
//! 
//! // With flags
//! ifop::create_folder("c:\\", "folder", Some(
//!     windows::Win32::UI::Shell::FOF_ALLOWUNDO |
//!     windows::Win32::UI::Shell::FOF_NORECURSION
//! )).unwrap();
//! ```
//! 
//! ## Command Usage
//! 
//! Compile examples
//! ```console
//! cargo build --examples
//! ```
//! 
//! `Copy` single file
//! ```console
//! ifop copy --src <filename|folder> --dest <folder>
//! ```
//! 
//! `Copy` multiple files
//! ```console
//! ifop copy --src <filename|folder>,<filename|folder>... --dest <folder>
//! ```
//! 
//! `Move` single file
//! ```console
//! ifop move --src <filename|folder> --dest <folder>
//! ```
//! 
//! `Move` multiple files
//! ```console
//! ifop move --src <filename|folder>,<filename|folder>... --dest <folder>
//! ```
//! 
//! `Rename` single file
//! ```console
//! ifop rename --src <filename|folder> --dest <folder>
//! ```
//! 
//! `Rename` multiple files
//! ```console
//! ifop rename --src <filename|folder>,<filename|folder>... --dest <folder>
//! ```
//! 
//! `Delete` single file
//! ```console
//! ifop delete --target <filename|folder>
//! ```
//! 
//! `Delete` multiple files
//! ```console
//! ifop delete --target <filename|folder>,<filename|folder>...
//! ```
//! 
//! Create file
//! ```console
//! ifop new-file --root <root_path> --name <name>
//! ```
//! 
//! Create folder
//! ```console
//! ifop new-folder --root <root_path> --name <name>
//! ```
//! 
//! With `--flags`
//! ```console
//! ifop <command> [options] --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
//! ```

use windows::Win32::UI::Shell::*;
use windows::Win32::System::Com::{ CoCreateInstance, CLSCTX_ALL};
use windows::Win32::Storage::FileSystem::*;
use windows::core::*;
use windows_core::Result;

unsafe fn get_item(target: &str) -> Result<IShellItem>{
    SHCreateItemFromParsingName( &HSTRING::from(target), None)
}

unsafe fn get_operation(op: Option<FILEOPERATION_FLAGS>) -> Result<IFileOperation> {
    let result:IFileOperation = CoCreateInstance(&FileOperation, None, CLSCTX_ALL)?;
    if let Some(flags) = op {
        result.SetOperationFlags(flags)?
    }
    Ok(result)
}

/// ### Copy multiple `files`
/// ```
/// let mut folders = vec![
///     "c:\\src\\file1.txt",
///     "c:\\src\\file2.txt"
/// ];
/// 
/// match ifop::copy_files(folders, "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
/// 
/// ### Copy multiple `folders`
/// ```
/// let mut folders = vec![
///     "c:\\src\\folder1",
///     "c:\\src\\folder2"
/// ];
/// 
/// match ifop::copy_files(folders, "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
pub fn copy_files(src: Vec<&str>, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        for dir in src {
            operation.CopyItem(&get_item(dir)?, &get_item(dest)?, None, None)?;
        }
        operation.PerformOperations()
    }
}

/// ### Copy one `file`
/// ```rust
/// match ifop::copy_file("c:\\src\\file1.txt", "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
/// 
/// ### Copy one `folder`
/// ```rust
/// match ifop::copy_file("c:\\src\\folder1", "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
pub fn copy_file(src: &str, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.CopyItem(&get_item(src)?,&get_item(dest)?, None, None)?;
        operation.PerformOperations()
    }
}


/// ### Delete one `file`
/// ```rust
/// match ifop::delete_file("c:\\file.txt", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// 
/// ### Delete one `folder`
/// ```rust
/// match ifop::delete_file("c:\\folder1", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
pub fn delete_file(target: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.DeleteItem(&get_item(target)?, None)?;
        operation.PerformOperations()
    }
}

/// ### Delete multiple `files`
/// ```rust
/// let mut files = vec![
///     "c:\\file1.txt",
///     "c:\\file2.txt"
/// ];
/// 
/// match ifop::delete_files(files, None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// 
/// ### Delete multiple `folders`
/// ```rust
/// let mut folders = vec![
///     "c:\\folder1",
///     "c:\\folder2"
/// ];
/// 
/// match ifop::delete_files(folders, None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// ```
pub fn delete_files(targets: Vec<&str>, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        for target in targets {
            operation.DeleteItem(&get_item(target)?, None)?;
        }
        operation.PerformOperations()
    }
}

/// ### Rename one `file`
/// ```rust
/// match ifop::rename_file("c:\\file1.txt", "file2.txt", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// ### Rename one `folder`
/// ```rust
/// match ifop::rename_file("c:\\folder1", "folder2", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn rename_file(src: &str, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.RenameItem(&get_item(src)?, &HSTRING::from(dest), None)?;
        operation.PerformOperations()
    }
}

/// ### Rename multiple `files`
/// ```rust
/// let mut files = vec![
///     "c:\\folder1\\file1.txt", 
///     "c:\\folder1\\file1.txt"
/// ];
/// 
/// match ifop::rename_files(files, "file2.txt", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// 
/// ### Rename multiple `folders`
/// ```rust
/// let mut files = vec![
///     "c:\\folder1\\folder1", 
///     "c:\\folder2\\folder1"
/// ];
/// 
/// match ifop::rename_files(files, "folder2", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn rename_files(targets: Vec<&str>, new_name: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        for target in targets {
            operation.RenameItem(&get_item(target)?, &HSTRING::from(new_name), None)?;
        }
        operation.PerformOperations()
    }
}

/// ### Move one `file`
/// ```rust
/// match ifop::move_file("c:\\src\\file.txt", "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// ### Move one `folder`
/// ```rust
/// match ifop::move_file("c:\\src\\folder", "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn move_file(src: &str, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.MoveItem(&get_item(src)?, &get_item(dest)?, None, None)?;
        operation.PerformOperations()
    }
}


/// ### Move multiple `files`
/// ```rust
/// let mut files = vec![
///     "c:\\folder1", 
///     "c:\\folder2"
/// ];
/// 
/// match ifop::move_files(files, "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
/// ### Move multiple `folders`
/// ```rust
/// let mut files = vec![
///     "c:\\src\\folder1", 
///     "c:\\src\\folder2"
/// ];
/// 
/// match ifop::move_files(files, "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn move_files(src: Vec<&str>, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        for dir in src {
            operation.MoveItem(&get_item(dir)?, &get_item(dest)?, None, None)?;
        }
        operation.PerformOperations()
    }
}

/// ### Create `folder`
/// ```rust
/// match ifop::create_folder("c:\\", "folder1", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn create_folder(target: &str, name: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.NewItem(
            &get_item(target)?, 
            FILE_ATTRIBUTE_DIRECTORY.0, 
            &HSTRING::from(name), 
            None, None)?;
        operation.PerformOperations()
    }
}

/// ### Create `folder`
/// ```rust
/// match ifop::create_file("c:\\", "file1.txt", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn create_file(target: &str, name: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let operation = get_operation(flags)?;

        operation.NewItem(
            &get_item(target)?, 
            FILE_ATTRIBUTE_NORMAL.0, 
            &HSTRING::from(name), 
            None, None)?;
        operation.PerformOperations()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};

    #[test]
    fn test_new_folder() {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let curr_dir = std::env::current_dir().unwrap();
        let root_dir = &format!("{}\\test", curr_dir.to_str().unwrap());
        assert_eq!(create_folder(root_dir, "new_folder", None),  Ok(()));
        assert_eq!(delete_file(&format!("{}\\new_folder", root_dir), None),  Ok(()));
    }

    #[test]
    fn test_new_file() {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let curr_dir = std::env::current_dir().unwrap();
        let root_dir = &format!("{}\\test", curr_dir.to_str().unwrap());
        assert_eq!(create_file(root_dir, "new_file", None),  Ok(()));
        assert_eq!(delete_file(&format!("{}\\new_file", root_dir), None),  Ok(()));
    }

    #[test]
    fn test_copy() {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let curr_dir = std::env::current_dir().unwrap();
        let root_dir = &format!("{}\\test", curr_dir.to_str().unwrap());
        let folder = &format!("{}\\test_copy", root_dir);
        let src = &format!("{}\\src", folder);
        let dest = &format!("{}\\dest", folder);
        let src_folder = &format!("{}\\folder", src);
        assert_eq!(create_folder(root_dir, "test_copy", None),  Ok(()));
        assert_eq!(create_folder(folder, "src", None),  Ok(()));
        assert_eq!(create_folder(folder, "dest", None),  Ok(()));
        assert_eq!(create_folder(src, "folder", None),  Ok(()));
        assert_eq!(create_file(src_folder, "file1", None),  Ok(()));
        assert_eq!(create_file(src_folder, "file2", None),  Ok(()));
        assert_eq!(copy_file(src_folder, dest, None),  Ok(()));
        assert_eq!(delete_file(folder, None),  Ok(()));
    }

    #[test]
    fn test_move() {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let curr_dir = std::env::current_dir().unwrap();
        let root_dir = &format!("{}\\test", curr_dir.to_str().unwrap());
        let folder = &format!("{}\\test_move", root_dir);
        let src = &format!("{}\\src", folder);
        let dest = &format!("{}\\dest", folder);
        let src_folder = &format!("{}\\folder", src);
        assert_eq!(create_folder(root_dir, "test_move", None),  Ok(()));
        assert_eq!(create_folder(folder, "src", None),  Ok(()));
        assert_eq!(create_folder(folder, "dest", None),  Ok(()));
        assert_eq!(create_folder(src, "folder", None),  Ok(()));
        assert_eq!(create_file(src_folder, "file1", None),  Ok(()));
        assert_eq!(create_file(src_folder, "file2", None),  Ok(()));
        assert_eq!(move_file(src_folder, dest, None),  Ok(()));
        assert_eq!(delete_file(folder, None),  Ok(()));
    }


    #[test]
    fn test_rename() {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

        let curr_dir = std::env::current_dir().unwrap();
        let root_dir = &format!("{}\\test", curr_dir.to_str().unwrap());
        let folder = &format!("{}\\test_rename", root_dir);
        let src = &format!("{}\\src", folder);
        assert_eq!(create_folder(root_dir, "test_rename", None),  Ok(()));
        assert_eq!(create_folder(folder, "src", None),  Ok(()));
        assert_eq!(rename_file(src, "dest", None),  Ok(()));
        assert_eq!(delete_file(folder, None),  Ok(()));
    }
}