use std::env;
use std::fs;

//'.' -> empty
//x -> wall
//o -> box
//p -> player
//F -> finish

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        println!("give the file as argument");
        return;
    }
    let file_path = &args[1];

    let map = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let map = map.bytes();
    let width = map.clone().take_while(|x| *x != b'\n').count();
    let mut map : Vec<u8> = map.filter(|x| *x != b'\n').collect();
    let height = map.len()/width;

    println!("map size: {}, {}", width, height);

    let mut p = (0, 0);
    let mut f : Vec<(i32, i32)> = vec![];
    for (i, c) in map.iter_mut().enumerate() {
        if *c == b'p' {
            p = ((i%height) as i32, (i/width ) as i32);
        }
        if *c == b'f' {
            f.push(((i%height) as i32, (i/width ) as i32));
            *c = b'.';
        }
    }

    let idx = |x: i32, y: i32| {
        x + y*width as i32
    };

    let tile = |x: i32, map: &Vec<u8>| -> u8 {
        if x < 0 || x >= map.len() as i32 {
            b'x'
        } else {
            map[x as usize]
        }
    };

    let step = |next_step: (i32, i32), map: &mut Vec<u8>, p: &mut (i32, i32) | -> (bool, bool) {
        let (nx, ny) = next_step;
        if tile(idx(p.0 + nx, p.1 + ny), map) == b'.' {
            p.0 += nx;
            p.1 += ny;
            (true, false)
        } else if tile(idx(p.0 + nx, p.1 + ny), map) == b'o' &&
                    tile(idx(p.0 + 2*nx, p.1 + 2*ny), map) == b'.' {
            p.0 += nx;
            p.1 += ny;
            map[idx(p.0, p.1) as usize] = b'.';
            map[idx(p.0 + nx, p.1 + ny) as usize] = b'o';
            (true, true)
        } else {
            (false, false)
        }
    };

    let unstep = |last_step: (i32, i32, bool), map: &mut Vec<u8>, p : &mut (i32, i32)| {
        let (lx, ly, push) = last_step;
        if push {
            map[idx(p.0 + lx, p.1 + ly) as usize] = b'.';
            map[idx(p.0, p.1) as usize] = b'o';
        } else {
            map[idx(p.0, p.1) as usize] = b'.';
        }
        p.0 -= lx;
        p.1 -= ly;
    };
    

    let circle = |s| match s {
        (0,  0) => Some((1,  0)),
        (1,  0) => Some((0, -1)),
        (0, -1) => Some((-1, 0)),
        (-1, 0) => Some((0,  1)),
        (0,  1) => None,
        _ => panic!()
    };

    let mut step_cnt = 1;
    let mut step_list : Vec<(i32, i32, bool)> = vec!();
    let mut result : Vec<(i32, i32, bool)> = vec!();
    while step_cnt < 100 {
        let mut next_step = (0, 0);
        //println!("new --- {}", step_cnt);
        loop {
            //print!("player: {}, {} -> ", p.0, p.1);
            if f.iter().any(|a| *a == p) {
                if step_list.len() < result.len() || result.len() == 0 {
                    result = step_list.clone();
                    //print!("pb! ");
                }
                //print!("found flag");
                //step back
                if step_list.len() == 0 {
                    break;
                }
                let last_step = *step_list.last().unwrap();
                unstep(last_step, &mut map, &mut p);
                step_list.pop();
                next_step = (last_step.0, last_step.1);
            } else if step_list.len() >= step_cnt {
                //print!("reached step limit");
                //step back
                if step_list.len() == 0 {
                    break;
                }
                let last_step = *step_list.last().unwrap();
                unstep(last_step, &mut map, &mut p);
                step_list.pop();
                next_step = (last_step.0, last_step.1);
            } else {
                if circle(next_step).is_some() {
                    next_step = circle(next_step).unwrap();
                    //print!("step({}, {}): ", next_step.0, next_step.1);
                    let step_result = step(next_step, &mut map, &mut p);
                    if step_result.0 {
                        //print!("success");
                        step_list.push((next_step.0, next_step.1, step_result.1));
                        next_step = (1, 0);
                    } else {
                        //print!("failed");
                    }
                } else {
                    //print!("just stepped back");
                    //step_back
                    if step_list.len() == 0 {
                        break;
                    }
                    let last_step = *step_list.last().unwrap();
                    unstep(last_step, &mut map, &mut p);
                    step_list.pop();
                    next_step = (last_step.0, last_step.1);
                }
            }
            //println!();
        }
        //println!();
        if result.len() != 0 {
            break;
        }
        step_cnt += 1;
    }

    for c in result {
        print!("{}", match (c.0, c.1) {
            (1,  0) => 'r',
            (0, -1) => 'u',
            (-1, 0) => 'l',
            (0,  1) => 'd',
            _ => '?'
        });
    }
}

