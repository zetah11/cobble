use anyhow::{anyhow, Result};
use cobble::nodes::{LInvert, Math, Mix, Operation, Random, Voronoi};
use cobble::{Combiner, Config, Generator, Metric, Node, Processor, Texture};
use image::RgbaImage;

fn main() -> Result<()> {
    let config = Config::new(256, 256);

    let mut smooth = Voronoi::new(50).with_metric(Metric::Euclidian);
    let mut linvert = LInvert;
    let mut plucky = Voronoi::new(20).with_metric(Metric::Euclidian);
    let mut mix = Math::new(Operation::Multiply);
    let mut random = Random;
    let mut mix2 = Mix::new(0.05);

    smooth.init(&config);
    linvert.init(&config);
    plucky.init(&config);
    mix.init(&config);
    random.init(&config);
    mix2.init(&config);

    let mut buf1 = Texture::new(config.width, config.height);
    let mut buf2 = Texture::new(config.width, config.height);
    let mut buf3 = Texture::new(config.width, config.height);

    smooth.render(&config, &mut buf1);
    linvert.render(&config, &buf1, &mut buf2);
    plucky.render(&config, &mut buf3);

    mix.render(&config, (&buf2, &buf3), &mut buf1);

    random.render(&config, &mut buf2);
    mix2.render(&config, (&buf1, &buf2), &mut buf3);

    let img = RgbaImage::from_vec(
        config.width as u32,
        config.height as u32,
        buf3.pixels().flat_map(|pix| pix.as_rgba8_sat()).collect(),
    )
    .ok_or_else(|| anyhow!("incorrect image dimensions"))?;

    img.save("test.png")?;

    Ok(())
}
