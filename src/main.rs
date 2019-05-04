#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let result = get_coordinates(1, 1, 3, 3);
    println!("{:?}", result);
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
        if error2 > i32::abs(dy) {
            error -= dy;
            current_x += sx;
        } else if error2 < dx {
            error += dx;
            current_y += sy;
        }
        coordinates.push(Point { x: current_x, y: current_y });
    }
    coordinates
}

#[cfg(test)]
mod tests {
   use super::*;
 
    #[test]
    fn test_add() {
        assert!(get_coordinates(1, 1, 3, 3), [Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 2, y: 3 }]);
    }
}
