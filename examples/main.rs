use rayt_rs::scene_builder::SceneBuilder;

fn main() -> anyhow::Result<()> {
    // Safety: just for logging
    unsafe { std::env::set_var("RUST_LOG", "info") }
    env_logger::init();

    let (camera, world) = SceneBuilder::build("scene1.toml")?;

    camera.render(world);

    Ok(())
}
