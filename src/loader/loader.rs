use std::fs;

fn load_images() {
    let folder = "datasets/images";

    for entry in fs::read_dir(folder).unwrap() {
        let path = entry.unwrap().path();

        println!("Loading {:?}", path);

        // Pass this image to OpenCV
    }
}