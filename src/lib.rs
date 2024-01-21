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
//! ifop::copy_file("c:\\src\\file.text", "c:\\dest"， None).unwrap();
//! 
//! // With flags
//! ifop::copy_file("c:\\src\\file.text", "c:\\dest"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Copy` multiple files
//! ```rust
//! // No flags
//! ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Move` single file
//! ```rust
//! // No flags
//! ifop::move_file("c:\\src\\file.text", "c:\\dest"， None).unwrap();
//! 
//! // With flags
//! ifop::move_file("c:\\src\\file.text", "c:\\dest"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Move` multiple files
//! ```rust
//! // No flags
//! ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();
//! 
//! // With flags
//! ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Rename` single file
//! ```rust
//! // No flags
//! ifop::rename_file("c:\\src\\folder1", "folder2"， None).unwrap();
//! 
//! // With flags
//! ifop::rename_file("c:\\src\\folder1", "folder2"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Rename` multiple files
//! ```rust
//! // No flags
//! ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt", None).unwrap();
//! 
//! // With flags
//! ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Delete` single file
//! ```rust
//! // No flags
//! ifop::delete_file("c:\\src\\folder1"， None).unwrap();
//! // With flags
//! ifop::delete_file("c:\\src\\folder1"， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
//! ```
//! 
//! - `Delete` multiple files
//! ```rust
//! // No flags
//! ifop::delete_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], None).unwrap();
//! 
//! // With flags
//! ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"]， FOF_ALLOWUNDO|FOF_NORECURSION).unwrap();
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
//! With `--flags`
//! ```console
//! ifop <command> [options] --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
//! ```

use windows::Win32::UI::Shell::*;
use windows::Win32::System::Com::{ CoCreateInstance, CLSCTX_ALL};
use windows::core::*;
use windows_core::{ComInterface, Result};

unsafe fn get_item<T: ComInterface>(target: &str) -> Result<T>{
    SHCreateItemFromParsingName( &HSTRING::from(target), None)
}

unsafe fn get_items(targets: Vec<&str>) -> Result<IShellItemArray> {
    let mut file_idlists: Vec<*const Common::ITEMIDLIST> = Vec::new();

    for path in targets {
        file_idlists.push(SHSimpleIDListFromPath(&HSTRING::from(path)));
    }

    SHCreateShellItemArrayFromIDLists(file_idlists.as_mut_slice())
}
 
/// ### Copy multiple `files`
/// ```
/// let mut folders: Vec<&str> = Vec::new();
/// folders.push("c:\\src\\file1.txt");
/// folders.push("c:\\src\\file2.txt");
/// 
/// match ifop::copy_files(folders, "c:\\dest", None) {
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
/// ### Copy multiple `folders`
/// ```
/// let mut folders: Vec<&str> = Vec::new();
/// folders.push("c:\\src\\folder1");
/// folders.push("c:\\src\\folder2");
/// 
/// match ifop::copy_files(folders, "c:\\dest", None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn copy_files(src: Vec<&str>, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    if src.len() < 1 {
        return Ok(())
    }

    unsafe {
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let item_array:Result<IShellItemArray> = get_items(src.clone());

                match item_array {
                    Ok(items) => {
                        let de: Result<IShellItem> = SHCreateItemFromParsingName(&HSTRING::from(dest), None);
                        
                        match de {
                            Ok(dest_item) => {
                                if let Err(e) = operation.CopyItems(&items, &dest_item) {
                                    return Err(e)
                                }
                                operation.PerformOperations()
                            }
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}

/// ### Copy one `file`
/// ```rust
/// match ifop::copy_file("c:\\src\\file1.txt", "c:\\dest"， None) {
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
/// match ifop::copy_file("c:\\src\\folder1", "c:\\dest"， None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn copy_file(src: &str, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let r:Result<IFileOperation> = CoCreateInstance(
            &FileOperation, None, CLSCTX_ALL);
            
        match r {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let sr: Result<IShellItem> = get_item(src);
                let de: Result<IShellItem> = get_item(dest);
    
                match sr {
                    Ok(src_item) => {
                        match de {
                            Ok(dest_item) => {
                                let result = operation.CopyItem(
                                    &src_item, &dest_item, None, None);
            
                                if let Err(e) = result {
                                    return Err(e)
                                }
                            
                                operation.PerformOperations()
                            }
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
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
/// 
/// ```
pub fn delete_file(target: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let tgt:Result<IShellItem> = get_item(target);

                match tgt {
                    Ok(target_item) => {
                        if let Err(e) = operation.DeleteItem(&target_item, None) {
                            return Err(e)
                        }
                        operation.PerformOperations()
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}

/// ### Delete multiple `files`
/// ```rust
/// let mut files: Vec<&str> = Vec::new();
/// files.push("c:\\file1.txt");
/// files.push("c:\\file2.txt");
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
/// let mut folders: Vec<&str> = Vec::new();
/// folders.push("c:\\folder1");
/// folders.push("c:\\folder2");
/// 
/// match ifop::delete_files(folders, None) {
///     Ok(_) => {
///         println!("Success");
///     }
///     Err(e) => {
///         println!("{}", e);
///     }
/// }
/// 
/// ```
pub fn delete_files(targets: Vec<&str>, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let tgts: Result<IShellItemArray> = get_items(targets);

                match tgts {
                    Ok(target_items) => {
                        if let Err(e) = operation.DeleteItems(&target_items) {
                            return Err(e)
                        }
                        operation.PerformOperations()
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}

/// ### Rename one `file`
/// ```rust
/// match ifop::rename_file("c:\\file1.txt", "file2.txt") {
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
/// match ifop::rename_file("c:\\folder1", "folder2") {
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
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let tgts: Result<IShellItem> = get_item(src);

                match tgts {
                    Ok(target_item) => {
                        if let Err(e) = operation.RenameItem(&target_item, &HSTRING::from(dest), None) {
                            return Err(e)
                        }
                        operation.PerformOperations()
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}

/// ### Rename multiple `files`
/// ```rust
/// let mut files: Vec<&str> = Vec::new();
/// files.push("c:\\folder1\\file1.txt");
/// files.push("c:\\folder2\\file1.txt");
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
/// let mut files: Vec<&str> = Vec::new();
/// files.push("c:\\folder1\\folder1");
/// files.push("c:\\folder2\\folder1");
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
pub fn rename_files(files_name: Vec<&str>, new_name: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let tgts: Result<IShellItemArray> = get_items(files_name);

                match tgts {
                    Ok(target_items) => {
                        if let Err(e) = operation.RenameItems(&target_items, &HSTRING::from(new_name)) {
                            return Err(e)
                        }
                        operation.PerformOperations()
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
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
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let sr: Result<IShellItem> = get_item(src);
                let de: Result<IShellItem> = get_item(dest);

                match sr {
                    Ok(src_item) => {
                        match de {
                            Ok(dest_item) => {
                                if let Err(e) = operation.MoveItem(&src_item, &dest_item, None, None) {
                                    return Err(e)
                                }
                                operation.PerformOperations()
                            }
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}


/// ### Move multiple `files`
/// ```rust
/// let mut files: Vec<&str> = Vec::new();
/// files.push("c:\\folder1");
/// files.push("c:\\folder2");
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
/// let mut files: Vec<&str> = Vec::new();
/// files.push("c:\\src\\folder1");
/// files.push("c:\\src\\folder2");
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
pub fn move_files(files: Vec<&str>, dest: &str, flags: Option<FILEOPERATION_FLAGS>) -> Result<()> {
    unsafe {
        let result:Result<IFileOperation> = CoCreateInstance(&FileOperation, None, CLSCTX_ALL);

        match result {
            Ok(operation) => {

                if let Some(f) = flags {
                    if let Err(e) = operation.SetOperationFlags(f) {
                        return Err(e)
                    }
                }

                let fls: Result<IShellItemArray> = get_items(files);
                let de: Result<IShellItem> = get_item(dest);
    
                match fls {
                    Ok(files_array) => {
                        match de {
                            Ok(dest_item) => {
                                if let Err(e) = operation.MoveItems(&files_array, &dest_item) {
                                    return Err(e)
                                }
                                operation.PerformOperations()
                            }
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
}
