//use std::cmp::PartialEq;
#[derive(Debug)]


struct Point {
    x: i32,
    y: i32
}
/*
#[derive(PartialEq)]
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
*/

fn main() {
    //let result = get_coordinates(1, 1, 69, 28);
    let result = get_coordinates(10, 10, 1, 20);
    draw_line(result, 70, 30);
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
        println!("x: {} y: {}", current_x, current_y);
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
    println!("{:?}", coordinates);
    coordinates
}

fn draw_line(line: std::vec::Vec<Point>, width: i32, height: i32) {
    //println!("{:?}", line);

    let total_n_points = line.len();
    println!("{}", total_n_points);
    let mut this_point = 0;  

    for col in 0..height {
        let mut col_as_string: String = "".to_string();

        for row in 0..width {
            let mut temp_row = false;
            for i in this_point..total_n_points {
                if line[i].x == row && line[i].y == col {
                    col_as_string = format!("{}{}", col_as_string, "❖");
                //println!("{:?}", line[this_point]);
                    temp_row = true;
                    this_point += 1;
                    break;
                
                    } 
            }

            if col == 0 || col == (height - 1) || row == 0 || row == (width - 1) {
                col_as_string = format!("{}{}", col_as_string, "☗");
            } else if temp_row == false {
                col_as_string = format!("{}{}", col_as_string, ".");
            }
            
            /*else {
                col_as_string = format!("{}{}", col_as_string, ".");
            }*/
            
        }
        println!("{}", col_as_string);

    }

/*
    for point in line.iter() {
        let point_x:usize = point.x as usize;
        let string_before = " ".repeat(point_x);  
        println!("{}x", string_before);   
    }
*/
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
