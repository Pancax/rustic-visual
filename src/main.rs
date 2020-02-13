#[macro_use]
extern crate glium;

use core::borrow::Borrow;
use glium::{glutin, Surface};
use std::{env, fs};
use hound::{SampleFormat, Sample};

fn main() {

    implement_vertex!(Vertex, position);
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();



    let mut readerino = hound::WavReader::open("C:\\Users\\dakot\\Videos\\testpianokey.wav").expect("File problem");
    let wave_file = readerino.spec().clone();

    let duration = readerino.duration().clone();

    let mut sample_iter = readerino.samples::<i16>();
    let mut channel1_graph: Vec<Vertex> = Vec::new();
    let mut channel2_graph: Vec<Vertex> = Vec::new();
    //-1 second
    let mut xpos:f32 = -1.0;
    //normalize is calculated off of 2.0/(total duration in seconds of file)
    //why? Well our range is -1 to 1 so we need to normalize each tick to add in that range
    //xcount is normalize/samplerate
    let normalize = 2.0/((duration as f32)/(wave_file.sample_rate as f32));

    let ynormalize = i32::pow(2,15) as f32;

    let xcount = normalize/(wave_file.sample_rate as f32);
    let mut channel=false;
    //normal y stuff normalized (y.unwrap() as f32)/(ynormalize)
    // y stuff with sin in there
    loop {
        match sample_iter.next() {

            Some(y) =>{

                let yvalue = (y.unwrap() as f32) /ynormalize;
                if !channel {
                    channel1_graph.push(Vertex { position: [xpos,  yvalue]});
                }else{
                    channel2_graph.push(Vertex {position: [xpos,  yvalue]});
                }
            },
            None => {break}

        }
        // add 1/sample_rate so that it does stuff
        if !channel {
            xpos += xcount;

        }
        channel=!channel;
    }



    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main(){
        gl_Position = vec4(position, 0.0, 1.0);
    }"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main(){
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }"#;
    let fragment_shader_src2 = r#"
    #version 140

    out vec4 color;

    void main(){
        color = vec4(1.0, 0.0, 1.0, 1.0);
    }"#;



    let mut closed =false;
    let mut countOffset = 0.00;
    let experiment:f32 = 1.0;
    let shape = channel1_graph;
    let shape2 = channel2_graph;
    while !closed{
/*
        let mut shape: Vec<Vertex> = Vec::new();
        let mut count: f32 = -3.14 + countOffset;
        let mut tracker: i32 = 0;
        let mut x_pos: f32 = -1.0;
        let mut last_vertex: Vertex = Vertex { position: [x_pos, (experiment*count.sin())/2.0] };
        while !(tracker == 6283) {
            shape.push(last_vertex);
            count += 0.001*experiment;
            tracker += 1;
            x_pos += 0.000318319274232;
            let y = (experiment*count.sin())/2.0;
            let new_vertex = Vertex { position: [x_pos, y] };
            shape.push(new_vertex);
            last_vertex = new_vertex;
        }
        countOffset += 0.001;
        if countOffset >= 6.283 {
            countOffset = 0.00;
        }
        */
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let program2 = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src2, None).unwrap();

        let mut target = display.draw();
        target.clear_color(0.0,0.0, 1.0 , 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

        target.draw(&vertex_buffer2, &indices, &program2, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();


        target.finish().unwrap();

        events_loop.poll_events(|ev|{
            match ev{
                glutin::Event::WindowEvent { event, ..} => match event{
                    glutin::WindowEvent::CloseRequested => {closed = true; },
                    _ => (),
                },
                _ => (),
            }
        });
    }

}

#[derive(Copy, Clone, Debug)]
struct Vertex{
    position: [f32; 2],
}