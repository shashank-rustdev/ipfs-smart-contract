use ipfs::display::display_file_content;
use ipfs::upload::upload_file;

mod configuration_parameters;
mod ipfs;

#[tokio::main]
async fn main() {
    let app_name = "ipfs_smart_contract";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    let cid = upload_file(config_param.file_path()).await;

    display_file_content(&cid).await;
}
