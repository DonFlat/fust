use std::error::Error;
use std::fs::OpenOptions;
use csv::WriterBuilder;

pub fn generate_test_size(min: usize, max: usize, num_points: usize) -> Vec<usize> {
    let min: f64 = 10.0;  // Minimum vector size
    let max: f64 = 10_000_000.0;  // Maximum vector size
    let num_points = 20;  // Number of points

    let log_min_size = min.log10();
    let log_max_size = max.log10();
    let step = (log_max_size - log_min_size) / (num_points - 1) as f64;

    let mut sizes = Vec::new();

    for i in 0..num_points {
        let log_size = log_min_size + step * i as f64;
        let size = 10f64.powf(log_size);
        sizes.push(size.round() as usize);  // Rounded to nearest whole number if necessary
    }
    return sizes;
}

pub fn powers_of_two(n: u32) -> Vec<u32> {
    (0..n).map(|i| 2_u32.pow(i)).collect()
}

pub fn append_to_csv(file_path: &str, vector_size: usize, repetitions: &Vec<f64>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    let mut row = vec![vector_size.to_string()];
    row.extend(repetitions.iter().map(|&val| val.to_string()));

    writer.write_record(&row)?;
    writer.flush()?;

    Ok(())
}