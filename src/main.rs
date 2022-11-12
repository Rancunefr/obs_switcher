use std::env;
use anyhow::Result;
use obws::Client;
use obws::requests::scene_items::SetEnabled;
use obws::requests::scene_items::Id;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        std::process::exit(exitcode::OK);
    }

    let arg_scene = &args[1];
    let arg_source = &args[2];

    let client = Client::connect("localhost", 4455, Some("totototo")).await?;
    
    let scenes = client.scenes();
    let current_scene = scenes.current_program_scene().await?;

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
            scene: &current_scene,
            item_id: item_id,
            enabled: !scene_items.enabled( &current_scene, item_id ).await?,
        })
        .await?;

    Ok(())
}
