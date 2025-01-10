use std::collections::HashMap;
use pyo3::prelude::*;

#[pyclass]
pub struct Discombobugraph;

#[pymethods]
impl Discombobugraph {
    #[new]
    pub fn new() -> Self {
        Discombobugraph
    }

    pub fn run(&self, bitstream: Vec<u8>) -> Vec<f64> {
        println!("Bitstream length: {}", bitstream.len());
        if bitstream.len() < 2 {
            panic!("Bitstream too short for analysis!");
        }
        let bitstream_bits = self.to_bitstream(&bitstream); // Convert bytes to bits
        let bitstream_bits_len = bitstream_bits.len(); // Length in bits
    
        let lags = [
            1,
            4,
            8,
            bitstream_bits_len / 10,
            bitstream_bits_len / 4,
            bitstream_bits_len / 2,
        ].iter()
        .copied()
        .filter(|&lag| lag < bitstream_bits_len) // Filter valid lags
        .collect::<Vec<usize>>();
    
        let mut results = vec![
            self.shannon_entropy(&bitstream), // Only Z-score
            self.frequency_test(&bitstream),
            self.runs_test(&bitstream),
            self.serial_test(&bitstream, 2),
            self.chi_square_test(&bitstream),
        ];

        for &lag in &lags {
            if lag < bitstream_bits_len {
                results.push(self.autocorrelation_test(&bitstream, lag)); // Extract Z-score
            } else {
                results.push(0.0);
            }
        }

        results
    }    

    fn to_bitstream(&self, input: &[u8]) -> Vec<u8> {
        input.iter().flat_map(|&byte| {
            (0..8).rev().map(move |i| (byte >> i) & 1)
        }).collect()
    }

    pub fn shannon_entropy(&self, bitstream: &[u8]) -> f64 {
        let mut counts = [0usize; 256];
        for &byte in bitstream {
            counts[byte as usize] += 1;
        }

        let len = bitstream.len() as f64;
        let entropy = counts.iter().fold(0.0, |entropy, &count| {
            if count == 0 {
                entropy
            } else {
                let p = count as f64 / len;
                entropy - p * p.log2()
            }
        });

        let max_entropy = len.log2();
        1.0 - entropy / max_entropy
    }

    // A high Z-score suggests a significant imbalance between the number of 1s and 0s, 
    // which may indicate non-randomness or bias in the bitstream.
    pub fn frequency_test(&self, bitstream: &[u8]) -> f64 {
        let sum: i32 = bitstream.iter().map(|&bit| if bit == 1 { 1 } else { -1 }).sum();
        (sum as f64).abs() / (bitstream.len() as f64).sqrt()
    }

    // A Z-score close to 0: Indicates that the number of runs is consistent with randomness.
    // A high Z-score: Suggests too many runs, which may indicate excessive alternation between 0 and 1.
    // A low Z-score: Suggests too few runs, which may indicate clustering of 0s or 1s.
    pub fn runs_test(&self, bitstream: &[u8]) -> f64 {
        let n = bitstream.len() as f64;
        let mut runs = 1;
        for i in 1..bitstream.len() {
            if bitstream[i] != bitstream[i - 1] {
                runs += 1;
            }
        }
        let pi = bitstream.iter().filter(|&&bit| bit == 1).count() as f64 / n;
        let expected_runs = 2.0 * n * pi * (1.0 - pi);
        ((runs as f64 - expected_runs).abs() / expected_runs.sqrt()).abs()
    }

    // Negative Z-score: Indicates fewer variations in n-bit sequences than expected.
    // Suggests the presence of repetitive patterns or dependencies.
    // Positive Z-score: Indicates more variation than expected.
    // Suggests an overly random sequence or excessive alternation.
    // Z-score near 0:
    // Indicates that the distribution of n-bit sequences is close to what is expected for randomness.
    pub fn serial_test(&self, bitstream: &[u8], n: usize) -> f64 {
        let mut counts = HashMap::new();
        for window in bitstream.windows(n) {
            *counts.entry(window.to_vec()).or_insert(0) += 1;
        }
        let total_windows = bitstream.len() - n + 1;
        counts.values().fold(0.0, |chi2, &count| {
            let expected = total_windows as f64 / 2f64.powi(n as i32);
            chi2 + ((count as f64 - expected).powi(2) / expected)
        })
    }

    // Negative Z-score: Indicates fewer deviations from the expected distribution.
    // Suggests the bitstream is closer to uniformly random.
    // Positive Z-score: Indicates greater deviations from the expected distribution.
    // Suggests the bitstream may have a bias or non-randomness.
    // Z-score near 0: Indicates that the observed counts of 0s and 1s are consistent with a uniform random distribution.
    pub fn chi_square_test(&self, bitstream: &[u8]) -> f64 {
        let bitstream_bits = self.to_bitstream(bitstream); // Convert to bitstream
        let len = bitstream_bits.len();
    
        let counts = bitstream_bits.iter().fold([0, 0], |mut acc, &bit| {
            acc[bit as usize] += 1;
            acc
        });
    
        counts.iter().fold(0.0, |chi2, &count| {
            let expected = len as f64 / 2.0;
            chi2 + ((count as f64 - expected).powi(2) / expected)
        })
    }    

    pub fn autocorrelation_test(&self, bitstream: &[u8], lag: usize) -> f64 {
        let bitstream_bits = self.to_bitstream(bitstream);
        let len = bitstream_bits.len();
        
        if lag >= len {
            return 0.0; // Return default values if lag is invalid
        }

        let n = len - lag;
        let and_count: f64 = (0..n)
            .map(|i| {
                let x = bitstream_bits[i];
                let y = bitstream_bits[i + lag];
                (x & y).count_ones() as f64
            })
            .sum();

        let nn = bitstream_bits.len() as f64;
        let f_and = and_count / nn;
        let f_bias = (bitstream_bits.iter().map(|&bit| bit.count_ones() as f64).sum::<f64>()) / nn;

        let z_score = if f_bias * f_bias <= 0.0 || f_bias >= 1.0 {
            0.0
        } else {
            (f_and - f_bias * f_bias) / (f_bias * (1.0 - f_bias)).sqrt()
        } * nn.sqrt();

        z_score
    }
    
}

#[pymodule]
fn discombobugraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Discombobugraph>()?;
    Ok(())
}