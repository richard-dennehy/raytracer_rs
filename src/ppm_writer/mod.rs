use crate::renderer::Canvas;

#[cfg(test)]
mod tests;

const MAX_COLOUR_VALUE: usize = 255;

pub fn write_ppm(canvas: &Canvas) -> String {
    let header = ppm_header(canvas.width(), canvas.height());
    let pixels = pixel_data(canvas);

    format!("{}{}", header, pixels)
}

fn ppm_header(width: usize, height: usize) -> String {
    // P3 - PPM version magic number
    // width height - output size
    // 255 - colour range (0 - 255)
    format!("P3\n{} {}\n{}\n", width, height, MAX_COLOUR_VALUE)
}

fn pixel_data(canvas: &Canvas) -> String {
    (0..canvas.height())
        .map(|y| {
            (0..canvas.width())
                .map(|x| canvas.get(x as _, y as _))
                .flat_map(|colour| {
                    vec![
                        ppm_colour_value(colour.red()),
                        ppm_colour_value(colour.green()),
                        ppm_colour_value(colour.blue()),
                    ]
                    .into_iter()
                })
                .map(|value| value.to_string())
                .fold((0, String::new()), |(line_length, acc), next| {
                    if line_length == 0 {
                        (next.len(), next)
                    } else if line_length + next.len() >= 70 {
                        // prevent line length exceeding 70
                        (next.len(), format!("{}\n{}", acc, next))
                    } else {
                        (line_length + next.len() + 1, format!("{} {}", acc, next))
                    }
                })
                .1 // discard line length counter
        })
        .fold(String::new(), |acc, next| format!("{}{}\n", acc, next))
}

fn ppm_colour_value(raw: f64) -> usize {
    if raw < 0.0 {
        0
    } else if raw >= 1.0 {
        MAX_COLOUR_VALUE
    } else {
        (raw * ((MAX_COLOUR_VALUE + 1) as f64)) as _
    }
}
