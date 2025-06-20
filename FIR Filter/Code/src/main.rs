use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Record{
    time : f32,
    pressure : f32,
}
struct Filter{
    coeff: Vec<f32>,
    buffer: Vec<f32>,
}

impl Filter{
    fn new (coeff: Vec<f32>) -> Self {
        let buffer = vec![0.0; coeff.len()];
        Filter { coeff, buffer }
    }

    fn filterrr(&mut self,input:f32) ->f32{
        for i in (1..self.buffer.len()).rev(){
            self.buffer[i] = self.buffer[i - 1];
        }
        self.buffer[0] = input; 
        self.coeff.iter().zip(&self.buffer).map(|(c, b)| c * b).sum()
    }
}

fn main() { 
    println!("Hello, world!");
    let coeffs = vec![0.1; 10];
    let mut filter = Filter::new (coeffs);
    let mut rdr = Reader::from_path(r"path.csv").expect("Failed to open CSV file");
    let mut write = File::create("filteredPressure.csv").expect("Failed to create output file");
    writeln!(write,"time,filteredPressure").unwrap();
    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed ");
        let filtered = filter.filterrr(record.pressure);
        println!("Time: {:.2} s, Pressure: {:.2} hPa ,Filtered Pressure : {:.2}", record.time, record.pressure,filtered );
        writeln!(write,"{:.2},{:.2}", record.time, filtered).expect("Failed to write to output file");
    }
}
