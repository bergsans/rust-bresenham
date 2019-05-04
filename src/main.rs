fn main() {
    let mut coordinates = Vec::new();
    let x1:i32 = 1;
    let y1:i32 = 1;
    let x2:i32 = 10;
    let y2:i32 = 5;
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
        coordinates.push([current_x, current_y]);
        let error2:i32 = 2 * error;
        if error2 > i32::abs(dy) {
            error -= dy;
            current_x += sx;
        } else if error2 < dx {
            error += dx;
            current_y += sy;
        }
    }
    println!("{:?}", coordinates);
}
