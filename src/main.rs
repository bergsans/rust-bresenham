extern crate clap;

struct Point {
    x: i32,
    y: i32
}

use clap::{ App,Arg };

fn main() {
    let args = App::new("draw line")
        .version("0.0.1")
        .about("Drawes a line")
        .arg(Arg::with_name("x1") 
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("y1")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("x2")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("y2")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("width")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("height")
             .takes_value(true)
             .required(true))
        .get_matches();
    
    let x1:i32 = match args.value_of("x1").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let y1:i32 = match args.value_of("y1").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let x2:i32 = match args.value_of("x2").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let y2:i32 = match args.value_of("y2").unwrap().parse() {
        Ok(n) => n,
        Err(_) => 1
    };
    let w:i32 = match args.value_of("width").unwrap().parse() {
        Ok(n) =>    {
                        if n >= 10 && n <= 100 {
                            n
                        } else {
                            70
                        }
                    },
        Err(_) => 70
    };
    let h:i32 = match args.value_of("height").unwrap().parse() {
        Ok(n) =>    {
                        if n > 10 && n < 100 {
                            n
                        } else {
                            30
                        }
                    },
        Err(_) => 30
    };

    
    let result = get_coordinates(x1, y1, x2, y2);
    draw_line(result, w, h);

    
}

fn get_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Point> {
    let mut coordinates = vec![];
    let dx:i32 = i32::abs(x2 - x1);
    let dy:i32 = i32::abs(y2 - y1);
    let sx:i32 = {
        if x1 < x2 {
            1
        } else {
            -1
        }
    };
    let sy:i32 ={
        if y1 < y2 {
            1
        } else {
            -1
        }
    };
    let mut error:i32 = dx - dy;
    let mut current_x:i32 = x1;
    let mut current_y:i32 = y1;
    coordinates.push(Point { x: current_x, y: current_y });
    while current_x != x2 && current_y != y2 {
        let error2:i32 = 2 * error;
        if error2 >= i32::abs(dy) {
            error -= dy;
            current_x += sx;
            coordinates.push(Point { x: current_x, y: current_y });
        } else if error2 <= i32::abs(dx) {
            error += dx;
            current_y += sy;
            coordinates.push(Point { x: current_x, y: current_y });
        }
    }
    coordinates
}

fn draw_line(line: std::vec::Vec<Point>, width: i32, height: i32) {
    for col in 0..height {
        for row in 0..width {
            let is_point_in_line = line.iter().any(| point| point.x == row && point.y == col);
            match is_point_in_line {
                true => print!("❖"),
                _ => {
                    if col == 0 || col == (height - 1) || row == 0 || row == (width - 1) {
                        print!("☗");
                    } else {
                        print!(".");
                    }
                }
            };
        }
        print!("\n");
    }
}

