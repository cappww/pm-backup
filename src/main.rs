use dotenv::dotenv;
use dropbox_sdk::files;
use dropbox_sdk::files::Metadata;
use dropbox_sdk::default_client::UserAuthDefaultClient;

fn main() {
    dotenv().ok();
    let auth = dropbox_sdk::oauth2::get_auth_from_env_or_prompt();
    let client = UserAuthDefaultClient::new(auth);

    dbx_list_files(&client, "");
    //dbx_write_file(&client, "");
}   

fn dbx_list_files (client: &UserAuthDefaultClient, path: &str) {
    let folder_path = &files::ListFolderArg::new(path.to_string());
    let result = files::list_folder(client, folder_path).unwrap().unwrap();

    for entry in result.entries {
        match entry {
            Metadata::File(meta) => {
                println!("{}", meta.name);
            }
            Metadata::Folder(meta) => {
                
            }
            Metadata::Deleted(meta) => {
            
            }
        }
    }
}

fn dbx_write_file(client: &UserAuthDefaultClient, path: &str) {

}
