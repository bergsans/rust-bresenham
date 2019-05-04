//#[derive(Debug)]

struct Point {
    x: i32,
    y: i32
}

fn main() {
    
    let result1 = get_coordinates(10, 10, 1, 20);
    draw_line(result1, 70, 30);

    let result2 = get_coordinates(1, 1, 69, 28);
    draw_line(result2, 70, 30);

    let result3 = get_coordinates(1, 28, 30, 15);
    draw_line(result3, 70, 30);
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

fn draw_line(mut line: std::vec::Vec<Point>, width: i32, height: i32) {

    for col in 0..height {
        let mut col_as_string: String = "".to_string();
        for row in 0..width {
            let mut is_there_a_point = false;
            for i in 0..line.len() {
                if line[i].x == row && line[i].y == col {
                    col_as_string = format!("{}{}", col_as_string, "❖");
                    is_there_a_point = true;
                    line.remove(i);
                    break;
                } 
            }

            if col == 0 || col == (height - 1) || row == 0 || row == (width - 1) {
                col_as_string = format!("{}{}", col_as_string, "☗");
            } else if is_there_a_point == false {
                col_as_string = format!("{}{}", col_as_string, ".");
            }
        }
        println!("{}", col_as_string);
    
}

#[cfg(test)]
mod tests {
   use super::*;

    // doesn't work
    #[test]
    fn test_coords() {
        let result = get_coordinates(1, 1, 2, 2);
     //   assert!(get_coordinates(1, 1, 2, 2), [Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 2, y: 3 }]);
    }
}
