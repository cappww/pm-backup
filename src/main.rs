extern crate cronjob;
use cronjob::CronJob;
use dotenv::dotenv;
use dropbox_sdk::files::{self, WriteMode};
use dropbox_sdk::files::{Metadata, CommitInfo};
use dropbox_sdk::default_client::UserAuthDefaultClient;
use std::fmt::Error;
use std::io::{self, Read};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    //dotenv().ok();


    let mut cron = CronJob::new("Dropbox Backup", on_cron);
    cron.minutes("2");
    cron.hours("*");
    cron.day_of_month("*");
    cron.day_of_week("*");
    cron.start_job();

    // let file_names = dbx_list_files(&client, "");
    // println!("{:?}", file_names)
}

fn on_cron(name: &str) {
    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    dbx_upload_file(&client, "/pass-keys.bcup");
}

fn dbx_list_files (client: &UserAuthDefaultClient, path: &str) -> Vec<String> {
    let folder_path = files::ListFolderArg::new(path.to_string());
    let result = files::list_folder(client, &folder_path).unwrap().unwrap();

    let mut file_names: Vec<String> = Vec::new();
    for entry in result.entries {
        match entry {
            Metadata::File(meta) => {
                file_names.push(meta.name);
            }
            Metadata::Folder(meta) => {
                
            }
            Metadata::Deleted(meta) => {
            
            }
        }
    }
    return file_names
}

fn dbx_upload_file(client: &UserAuthDefaultClient, path: &str) {
    let mut local_path = path.to_string();
    local_path.insert_str(0, ".");
    let file = File::open(local_path).expect("problem opening file");

    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("failed to read file");
    println!("buf.len(): {}", buf.len());
    
    let mut arg = CommitInfo::new(path.to_string());
    arg.mode = WriteMode::Overwrite;
    let result = files::upload(client, &arg, &buf)
        .expect("There was an error with uploading")
        .expect("There was an error with uploading");

    println!("result: {:?}", result);
}
