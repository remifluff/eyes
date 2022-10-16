use opencv::prelude::Mat;
use tflitec::interpreter::{Interpreter, Options};

use super::camera::{self, Camera};

pub struct Movenet {
    interpreter: Interpreter,
    data: Vec<f32>,
}
impl Movenet {
    pub fn new() -> Movenet {
        // load model and create interpreter
        let options = Options::default();
        let path = format!("resource/lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite");

        let interpreter =
            Interpreter::with_model_path(&path, Some(options)).unwrap();

        interpreter
            .allocate_tensors()
            .expect("Allocate tensors [FAILED]");

        let data = Vec::new();

        Movenet { interpreter, data }
    }
    pub fn update(&mut self, camera_data: Vec<u8>) {
        // set input (tensor0)

        self.interpreter.copy(&camera_data[..], 0).unwrap();

        // run interpreter
        self.interpreter.invoke().expect("Invoke [FAILED]");

        // get output
        self.data =
            self.interpreter.output(0).unwrap().data::<f32>().to_vec();

        // self.output_tensor = output_tensor.data::<f32>()
    }

    pub fn data(&self) -> &[f32] {
        self.data.as_ref()
    }
}
