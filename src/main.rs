use std::env;
use anyhow::Result;
use obws::Client;
use obws::requests::scene_items::SetEnabled;
use obws::requests::scene_items::Id;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        std::process::exit(exitcode::OK);
    }

    let arg_scene = &args[1];
    let arg_source = &args[2];
    let str_enable: bool = args[3].eq("true");

    let client = Client::connect("localhost", 4455, Some("totototo")).await?;
    
    let scene_items = client.scene_items();
    let item_id = scene_items
        .id(Id {
            scene: arg_scene,
            source: arg_source,
            search_offset: None,
        })
    .await?;

    scene_items
        .set_enabled(SetEnabled {
            scene: arg_scene,
            item_id: item_id,
            enabled: str_enable,
        })
        .await?;

    Ok(())
}
