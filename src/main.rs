use cpal::{EventLoop, Format, StreamData, UnknownTypeOutputBuffer};
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use std::i16;
use wasmtime::Func;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;

fn main() {
    let (s,r) = unbounded();
    std::thread::spawn(move || {
        let store = Store::default();
        let mut linker = Linker::new(&store);
        linker.define("heligen", "output", Func::wrap(&store, move |output: u64| {
            s.send(output).unwrap()
        })).unwrap();
        let module_file = std::fs::read("test_module.wasm").unwrap();
        let module = Module::new(store.engine(), module_file).unwrap();
        let instance = linker.instantiate(&module).unwrap();
        let start_func = instance.get_func("heligen_start").unwrap().get0::<()>().unwrap();
        start_func().unwrap();
    });
    receiver(r);
}

fn receiver(r: Receiver<u64>) {
    while let Ok(u) = r.recv() {
        println!("number was generated: {}", u);
    }
}

fn write_noise_wav() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut sample: i16 = 0;
    let mut vel = 0;
    let mut writer = hound::WavWriter::create("noise2.wav", spec).unwrap();
    writer.write_sample(sample).unwrap();
    for _ in 0..44100 * 2 * 60 {
        if rand::random() {
            vel += 1;
        } else {
            vel -= 1;
        }
        sample += vel;
        match writer.write_sample(sample) {
            Ok(_) => {}
            Err(_) => {
                println!("{}, {}", sample, vel);
                panic!();
            }
        };
    }
}

fn run_cpal() {
    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("No device available");
    let format = device.default_output_format().expect("no default format");
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);
    let mut flippo = true;
    let mut counter = 0;
    event_loop.run(move |_stream_id, stream_data| match stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            println!("f32");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = 1.0;
                } else {
                    *elem = -1.0;
                }
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            println!("i16");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = i16::max_value();
                } else {
                    *elem = i16::min_value();
                }
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            println!("u16");
            for elem in buffer.iter_mut() {
                counter += 1;
                if counter > 500 {
                    counter = 0;
                    flippo = !flippo;
                }
                if flippo {
                    *elem = u16::max_value();
                } else {
                    *elem = u16::min_value();
                }
            }
        }
        _ => {}
    });
}
