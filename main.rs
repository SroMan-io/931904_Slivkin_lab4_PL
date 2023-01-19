use std::io;

fn get_line_info(mut input:&String) -> (f32, f32, f32, f32) {

    let points: Vec<&str> = (&mut input).split_whitespace().collect();
    let xy1: Vec<&str> = points[0].split( ',' ).collect();
    let xy2: Vec<&str> = points[1].split( ',' ).collect();

    ( xy1[0].parse().unwrap(),  xy1[1].parse().unwrap(),  xy2[0].parse().unwrap(),  xy2[1].parse().unwrap() ) 
}

fn input_is_over(input:&String) -> bool{
     
    if input.trim() == "stop" {
        true   
    }
    else {false}
}

fn intersection_with_segment(px:&f32, py:&f32, x1:&f32, y1:&f32, x2:&f32, y2:&f32) -> bool{
    
    if ((px-x1)*(px-x1)+(py-y1)*(py-y1)).sqrt() + ((px-x2)*(px-x2)+(py-y2)*(py-y2)).sqrt() == ((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1)).sqrt() {
        true
    }  
    else {false}
}

fn main() {

    let mut is_over: bool = false;
    let mut min_distance = f32::MAX;
    let mut intersection_found: bool = false;

    let mut closest_seg_p1_x = 0.0;
    let mut closest_seg_p1_y = 0.0;
    let mut closest_seg_p2_x = 0.0;
    let mut closest_seg_p2_y = 0.0;
  
    println!("Введите координаты луча");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{ },
        Err(e) => {
            println!("Обнаружена ошибка {}!", {e});
        }
    };

    let ( ray_p1_x, ray_p1_y, ray_p2_x, ray_p2_y) = get_line_info(&input);
    
    let v = ray_p2_x - ray_p1_x;
    let w = ray_p2_y - ray_p1_y;

    println!( "Чтобы завершить работу программы и получить координаты ближайшего отрезка, введите команду 'stop'." );

    loop{
        if is_over == true {
            break;
        }

        println!("Введите координаты отрезка");

        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_)=>{ },
            Err(e) => {
                println!("Обнаружена ошибка {}!", {e});
            }
        };
        
        is_over = input_is_over(&input);

        if !is_over {
            let ( p1_x, p1_y, p2_x, p2_y) = get_line_info(&input);

            let a = p2_y - p1_y;
            let b = p1_x - p2_x;
            let c = -p1_x*p2_y+p1_y*p2_x;
          
            if a*v+b*w!=0.0 {
                let t = (-a*ray_p1_x-b*ray_p1_y-c)/(a*v+b*w);

                if t>=0.0 {
                    let intersection_p_x = ray_p1_x + v * t;
                    let intersection_p_y = ray_p1_y + w * t;

                    let cur_distance = ((intersection_p_x-ray_p1_x)*(intersection_p_x-ray_p1_x)+(intersection_p_y-ray_p1_y)*(intersection_p_y-ray_p1_y)).sqrt();

                    if min_distance>cur_distance && intersection_with_segment(&intersection_p_x, &intersection_p_y, &p1_x, &p1_y, &p2_x, &p2_y) {
                        closest_seg_p1_x = p1_x;
                        closest_seg_p1_y = p1_y;
                        closest_seg_p2_x = p2_x;
                        closest_seg_p2_y = p2_y;
                        min_distance = cur_distance;
                        intersection_found = true;
                    }
                }
            }
        }
    }
   
    if intersection_found {
        println!("{},{} {},{}", closest_seg_p1_x, closest_seg_p1_y, closest_seg_p2_x, closest_seg_p2_y);
    }

    else {
        println!();
    }  
}