extern crate cronjob;
use chrono::prelude::Utc;
use cronjob::CronJob;
use dotenv::dotenv;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use dropbox_sdk::files::{self, Metadata};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    dotenv().ok();

    // let mut cron = CronJob::new("Backup", on_cron);
    // cron.day_of_month("*");
    // cron.start_job();

    let mut cron = CronJob::new("Hello World", hello_world);
    cron.seconds("*");
    cron.minutes("*");
    cron.hours("*");
    cron.start_job();
}

fn hello_world(name: &str) {
    let utc = Utc::now();
    println!("{}: {:?}", name, utc);
}

fn on_cron(name: &str) {
    println!("Starting {}", name);
    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    dbx_upload_file(&client, "/pass-keys.bcup");
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
    let mut local_path = path.to_string();
    local_path.insert_str(0, ".");
    let file = File::open(local_path).expect("problem opening file");

    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("failed to read file");
    println!("buf.len(): {}", buf.len());

    let mut arg = files::CommitInfo::new(path.to_string());
    arg.mode = files::WriteMode::Overwrite;
    let result = files::upload(client, &arg, &buf)
        .expect("There was an error with uploading")
        .expect("There was an error with uploading");

    println!("result: {:?}", result);
}
