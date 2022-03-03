extern crate walkdir;
extern crate cronjob;
extern crate zip;
use chrono::prelude::Utc;
use cronjob::CronJob;
use dotenv::dotenv;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use dropbox_sdk::files::{self, Metadata};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::prelude;
use walkdir::WalkDir;
mod write_dir;

fn main() {
    dotenv().ok();

    // let mut cron = CronJob::new("Backup", on_cron);
    // cron.day_of_month("*");
    // cron.start_job();

    // let mut cron = CronJob::new("Dropbox Backup", on_cron);
    // cron.seconds("*");
    // cron.minutes("*");
    // cron.hours("*");
    // cron.start_job();

    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    // let file_names = dbx_list_files(&client,"");
    // print!("{:?}", file_names)


    let mut method = zip::CompressionMethod::Bzip2;

    write_dir::doit("./buttercup", "./buttercup.zip", method);  
}

fn hello_world(name: &str) {
    let utc = Utc::now();
    println!("{}: {:?}", name, utc);
}

fn on_cron(name: &str) {
    println!("Starting {}", name);
    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    // dbx_upload_file(&client, "/pass-keys.bcup");
    println!("Hello world");
}

fn dbx_list_files(client: &UserAuthDefaultClient, path: &str) -> Vec<String> {
    let folder_path = files::ListFolderArg::new(path.to_string());
    let result = files::list_folder(client, &folder_path).unwrap().unwrap();

    let mut file_names: Vec<String> = Vec::new();
    for entry in result.entries {
        match entry {
            Metadata::File(meta) => {
                file_names.push(meta.name);
            }
            Metadata::Folder(meta) => {}
            Metadata::Deleted(meta) => {}
        }
    }
    return file_names;
}

fn dbx_upload_file(client: &UserAuthDefaultClient, path: &str) {
    let file = File::open(path).expect("problem opening file");

    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("failed to read file");
    println!("buf.len(): {}", buf.len());

    let mut dbx_path = path.to_string();
    dbx_path.remove(0);

    let mut arg = files::CommitInfo::new(dbx_path);
    arg.mode = files::WriteMode::Overwrite;
    let result = files::upload(client, &arg, &buf)
        .expect("There was an error with uploading")
        .expect("There was an error with uploading");

    println!("result: {:?}", result);
}

fn dbx_upload_folder(client: &UserAuthDefaultClient, path: &str) {
    for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let file_name = file.path().display().to_string();
            dbx_upload_file(client, &file_name);
        }
    }
}