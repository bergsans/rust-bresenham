#[derive(Debug)]
struct Point {
    pos: [i32; 2]
}


fn main() {
    let result = get_coordinates(1, 1, 10, 5);
    println!("{:?}", result);
}

fn get_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Point> { //-> std::vec::Vec<i32>
    let mut coordinates = vec![];  //Vec::new();
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
    while current_x != x2 && current_y != y2 {
        coordinates.push(Point {pos: [current_x, current_y]});
        let error2:i32 = 2 * error;
        if error2 > i32::abs(dy) {
            error -= dy;
            current_x += sx;
        } else if error2 < dx {
            error += dx;
            current_y += sy;
        }
    }
    //println!("{:?}", coordinates);
    coordinates
}
