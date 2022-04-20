use std::fs;

pub fn create_image(pixel_array: &Vec<Vec<Vec<f64>>>) {
    let final_image_name = String::from("ray_tracing_engine_out");
    let ppm_string = pixel_array_to_ppm_string(&pixel_array);
    fs::write(format!("{}.ppm", final_image_name), ppm_string).unwrap();
}

fn pixel_array_to_ppm_string(pixel_array: &Vec<Vec<Vec<f64>>>) -> String {
    let (image_width, image_height) = (pixel_array.len(), pixel_array[0].len());
    let mut ppm_string = String::from(format!(
        "P3
        {w} {h}
        255\n",
        w = image_width,
        h = image_height
    ));
    for j in 0..image_height {
        for i in 0..image_width {
            for k in 0..3 {
                let pix = (pixel_array[i][j][k] * 255.0).round() as u8;
                ppm_string.push_str(pix.to_string().as_str());
                if k == 2 {
                    ppm_string.push_str("\n");
                } else {
                    ppm_string.push_str("  ");
                }
            }
        }
    }
    ppm_string
}
