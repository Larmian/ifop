use std::collections::HashMap;
use clap::{builder::Str, Parser, Subcommand};
use windows::Win32::UI::Shell::*;
use windows_core::Result;
use ifop::*;
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};

fn dump_result(res: Result<()>) {
    match res {
        Ok(_) => {
            print!("ok!");
        }
        Err(e) => {
            print!("{}", e)
        }
    }
}

fn apply_command(
    target: &String, 
    dest_op: Option<&String>, 
    command1: Option<fn (file: &str, dest: &str, flags:Option<FILEOPERATION_FLAGS>) -> Result<()>>, 
    command2: Option<fn (files: Vec<&str>, dest: &str, flags:Option<FILEOPERATION_FLAGS>) -> Result<()>>, 
    command3: Option<fn (file: &str, flags:Option<FILEOPERATION_FLAGS>) -> Result<()>>, 
    command4: Option<fn (files: Vec<&str>, flags:Option<FILEOPERATION_FLAGS>) -> Result<()>>,
    flags: Option<FILEOPERATION_FLAGS>
) {
    let target_files: Vec<&str> = target.split(",").collect();

    if let Some(dest) = dest_op {
        if let Some(cmd1) = command1 {
            if target_files.len() == 1 {
                dump_result(cmd1(target_files[0], dest, flags));
                return;
            }
        }

        if let Some(cmd2) = command2 {
            if target_files.len() > 1 {
                dump_result(cmd2(target_files, dest, flags));
                return;
            }
        }
    } else {
        if let Some(cmd3) = command3 {
            if target_files.len() == 1 {
                dump_result(cmd3(target_files[0], flags));
                return;
            }
        }

        if let Some(cmd4) = command4 {
            if target_files.len() > 1 {
                dump_result(cmd4(target_files, flags));
                return;
            }
        }
    }
    
    print!("No anything to do");
}

fn copy(src: &String, dest: &String, flags: Option<FILEOPERATION_FLAGS>) {
    { 
        apply_command(
            src, 
            Some(dest), 
            Some(copy_file), 
            Some(copy_files), 
            None, 
            None,
            flags
        );
    }
}

fn delete(target: &String, flags: Option<FILEOPERATION_FLAGS>) {
    { 
        apply_command(
            target, 
            None, 
            None, 
            None, 
            Some(delete_file), 
            Some(delete_files),
            flags
        );
    }
}

fn rename(target: &String, dest: &String, flags: Option<FILEOPERATION_FLAGS>) {
    { 
        apply_command(
            target, 
            Some(dest), 
            Some(rename_file), 
            Some(rename_files), 
            None, 
            None,
            flags
        );
    }
}

fn _move(src: &String, dest: &String, flags: Option<FILEOPERATION_FLAGS>) {
    { 
        apply_command(
            src, 
            Some(dest), 
            Some(move_file), 
            Some(move_files), 
            None, 
            None,
            flags
        );
    }
}

fn new_folder(target: &String, dest: &String, flags: Option<FILEOPERATION_FLAGS>) {
    {
        apply_command(
            target, 
            Some(dest), 
            Some(create_folder), 
            None, 
            None, 
            None, flags
        )
    }
}

fn new_file(target: &String, dest: &String, flags: Option<FILEOPERATION_FLAGS>) {
    {
        apply_command(
            target, 
            Some(dest), 
            Some(create_file), 
            None, 
            None, 
            None, flags
        )
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,

}
#[derive(Subcommand)]
enum Commands {

    /// Copy files from --src to --dest
    Copy {

        /// --src <filename|folder> or <filename|folder>,<filename|folder>,<filename|folder>...
        #[arg(short, long)]
        src: String,

        /// --dest <folder>
        #[arg(short, long)]
        dest: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    },

    /// Delete files from --target
    Delete {

        /// --target <filename|folder> or <filename|folder>,<filename|folder>...
        #[arg(short, long)]
        target: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    },

    /// Rename file from --src to --dest
    Rename {
        /// --src <filename|folder> or <filename|folder>,<filename|folder>,<filename|folder>...
        #[arg(short, long)]
        src: String,

        /// --dest "New name"
        #[arg(short, long)]
        dest: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    },

    /// Move files from --src to --dest
    Move {

        /// --src <filename|folder> or <filename|folder>,<filename|folder>,<filename|folder>...
        #[arg(short, long)]
        src: String,

        /// --dest <folder>
        #[arg(short, long)]
        dest: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    },

    /// Create folder --root <root_path> --name <name>
    NewFolder {
        /// --root <root_path>
        #[arg(short, long)]
        root: String,

        /// --name <folder_name>
        #[arg(short, long)]
        name: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    },

    /// Create file --root <root_path> --name <name>
    NewFile {
        /// --root <root_path>
        #[arg(short, long)]
        root: String,

        /// --name <file_name>
        #[arg(short, long)]
        name: String,

        /// --flags FOF_ALLOWUNDO|FOF_CONFIRMMOUSE|....
        /// More refer in microsoft doc
        #[arg(short, long)]
        flags: Option<String>
    }
}

fn query_flags(cli_flags: &Option<String>) -> Option<FILEOPERATION_FLAGS> {
    let flags_map: HashMap<&str, FILEOPERATION_FLAGS> = HashMap::from([
        ("FOF_ALLOWUNDO", FOF_ALLOWUNDO),
        ("FOF_CONFIRMMOUSE", FOF_CONFIRMMOUSE),
        ("FOF_FILESONLY", FOF_FILESONLY),
        ("FOF_MULTIDESTFILES", FOF_MULTIDESTFILES),
        ("FOF_NOCONFIRMATION", FOF_NOCONFIRMATION),
        ("FOF_NOCONFIRMMKDIR", FOF_NOCONFIRMMKDIR),
        ("FOF_NO_CONNECTED_ELEMENTS", FOF_NO_CONNECTED_ELEMENTS),
        ("FOF_NOCOPYSECURITYATTRIBS", FOF_NOCOPYSECURITYATTRIBS),
        ("FOF_NOERRORUI", FOF_NOERRORUI),
        ("FOF_NORECURSEREPARSE", FOF_NORECURSEREPARSE),
        ("FOF_NORECURSION", FOF_NORECURSION),
        ("FOF_NO_UI", FOF_NO_UI),
        ("FOF_RENAMEONCOLLISION", FOF_RENAMEONCOLLISION),
        ("FOF_SILENT", FOF_SILENT),
        ("FOF_SIMPLEPROGRESS", FOF_SIMPLEPROGRESS),
        ("FOF_WANTMAPPINGHANDLE", FOF_WANTMAPPINGHANDLE),
        ("FOF_WANTNUKEWARNING", FOF_WANTNUKEWARNING)
    ]);

    if let Some(flags_pattern) = cli_flags {
        let mut flags = FILEOPERATION_FLAGS(0);
        let flags_arr: Vec<&str> = flags_pattern.split("|").collect();

        for flag in flags_arr {
            if flags_map.contains_key(flag) {
                flags.0 = flags.0 | flags_map[flag].0;
            }
        }

        return Some(flags)
    }

    None
}

fn main() {
    let cli = Args::parse();
    let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    match &cli.command {
        Commands::Copy { src, dest, flags } => {
            copy(src, dest, query_flags(flags))
        }
        Commands::Delete { target, flags } => {
            delete(target, query_flags(flags))
        }
        Commands::Rename { src, dest, flags } => {
            rename(src, dest, query_flags(flags))
        }
        Commands::Move { src, dest, flags } => {
            _move(src, dest, query_flags(flags))
        }
        Commands::NewFolder { root, name, flags } => {
            new_folder(root, name, query_flags(flags))
        }
        Commands::NewFile { root, name, flags } => {
            new_file(root, name, query_flags(flags))
        }
    }
}