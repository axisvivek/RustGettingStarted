extern crate camera_capture;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let cam = camera_capture::create(1).unwrap();
    let cam = cam.fps(5.0).unwrap().start().unwrap();
    print_type_of(&cam);
    for _image in cam {
        print_type_of(&_image);
        println!("frame");
    }
    println!("done");
}