use terrain_gen_api::routes::height_map::Heightmap;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let mut heightmap = Heightmap::new(5, 0.2);
    heightmap.midpnt_displacement().await;
    //TODO: figure whjats the deal with height here
    // heightmap.render("heightmap.png");
    Ok(())
}
