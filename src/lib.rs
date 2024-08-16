use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TransposedMatrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

#[wasm_bindgen]
impl TransposedMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<i32>, rows: usize, cols: usize) -> TransposedMatrix {
        TransposedMatrix { data, rows, cols }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<i32> {
        self.data.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[wasm_bindgen(getter)]
    pub fn cols(&self) -> usize {
        self.cols
    }
}

#[wasm_bindgen]
pub fn transpose(matrix: Vec<i32>, rows: usize, cols: usize) -> TransposedMatrix {
    let mut transposed_data = vec![0; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            transposed_data[c * rows + r] = matrix[r * cols + c];
        }
    }
    TransposedMatrix::new(transposed_data, cols, rows)
}