- You can use this crate to copy|move|delete|rename file or folder like Explorer that.
- When you move a file to another folder, then you can undo or redo in explorer.
- Show progress window if this is big file.
- The `--flags` options Refer [`IFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperation)::[`SetOperationFlags`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setoperationflags)


## Installation

```console
$ cargo add ifop
```

## Crate Examples

- `Copy` single file
```rust
// No flags
ifop::copy_file("c:\\src\\file.text", "c:\\dest"， None).unwrap();

// With flags
ifop::copy_file("c:\\src\\file.text", "c:\\dest"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Copy` multiple files
```rust
// No flags
ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();

// With flags
ifop::copy_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Move` single file
```rust
// No flags
ifop::move_file("c:\\src\\file.text", "c:\\dest"， None).unwrap();

// With flags
ifop::move_file("c:\\src\\file.text", "c:\\dest"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Move` multiple files
```rust
// No flags
ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest", None).unwrap();

// With flags
ifop::move_files(vec!["c:\\src\\file1.txt", "c:\\src\\file2.txt"], "c:\\dest"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Rename` single file
```rust
// No flags
ifop::rename_file("c:\\src\\folder1", "folder2"， None).unwrap();

// With flags
ifop::rename_file("c:\\src\\folder1", "folder2"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Rename` multiple files
```rust
// No flags
ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt", None).unwrap();

// With flags
ifop::rename_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], "file3.txt"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Delete` single file
```rust
// No flags
ifop::delete_file("c:\\src\\folder1"， None).unwrap();
// With flags
ifop::delete_file("c:\\src\\folder1"， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- `Delete` multiple files
```rust
// No flags
ifop::delete_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"], None).unwrap();

// With flags
ifop::delete_files(vec!["c:\\src\\folder1\\file1.txt", "c:\\src\\folder2\\file2.txt"]， Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- Create file
```rust
// No flags
ifop::create_file("c:\\", "file.txt", None).unwrap();

// With flags
ifop::create_file("c:\\", "file.txt", Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

- Create folder
```rust
// No flags
ifop::create_folder("c:\\", "folder", None).unwrap();

// With flags
ifop::create_folder("c:\\", "folder", Some(
    windows::Win32::UI::Shell::FOF_ALLOWUNDO |
    windows::Win32::UI::Shell::FOF_NORECURSION
)).unwrap();
```

## Command Usage

Compile
```console
cargo build --examples
cd target/debug/examples
```

Command examples
```console
# Copy single file
ifop copy --src <filename|folder> --dest <folder>

# Copy multiple files
ifop copy --src <filename|folder>,<filename|folder>... --dest <folder>

# Move single file
ifop move --src <filename|folder> --dest <folder>

# Move multiple files
ifop move --src <filename|folder>,<filename|folder>... --dest <folder>

# Rename single file
ifop rename --src <filename|folder> --dest <folder>

# Rename multiple files
ifop rename --src <filename|folder>,<filename|folder>... --dest <folder>

# Delete single file
ifop delete --target <filename|folder>

# Delete multiple files
ifop delete --target <filename|folder>,<filename|folder>...

# Create file
ifop new-file --root <root_path> --name <name>

# Create folder
ifop new-folder --root <root_path> --name <name>

# With `--flags`
ifop <command> [options] --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
```

## Usage

```console
Commands:
  copy    Copy files from --src to --dest
  delete  Delete files from --target
  rename  Rename file from --src to --dest
  move    Move files from --src to --dest
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

|     Support                   |   Lower Version                               |
|   -------------               |   -------------                               |
|   Minimum supported client    |   Windows Vista [desktop apps only]           |
|   Minimum supported server    |   Windows Server 2008 [desktop apps only]     |
|   Target Platform             |   Windows                                     |