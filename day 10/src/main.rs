use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

const fn can_enter(field: char, direction: &Direction) -> bool {
    match direction {
        Direction::North => {
            if field == '|' || field == 'F' || field == '7' {
                return true
            }
            false
        }
        Direction::South => {
            if field == '|' || field == 'L' || field == 'J' {
                return true
            }
            false
        }
        Direction::East => {
            if field == '-' || field == '7' || field == 'J' {
                return true
            }
            false
        }
        Direction::West => {
            if field == '-' || field == 'F' || field == 'L' {
                return true
            }
            false
        }
    }
}

fn get_furthest_point(map: &Vec<Vec<char>>) -> usize {
    let mut start_position: (usize, usize) = (0, 0);

    'outer: for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.eq(&'S') {
                start_position = (x, y);
                break 'outer;
            }
        }
    }

    let mut path: HashMap<(usize, usize), usize> = HashMap::new();

    if start_position.1 > 0 && can_enter(map[start_position.1-1][start_position.0], &Direction::North) {
        start_position.1 -= 1;
        start_travel(map, start_position, Direction::North, &mut path);
        start_position.1 += 1;
    }
    if start_position.1 < map.len() - 1 && can_enter(map[start_position.1+1][start_position.0], &Direction::South) {
        start_position.1 += 1;
        start_travel(map, start_position, Direction::South, &mut path);
        start_position.1 -= 1;
    }
    if start_position.0 > 0 && can_enter(map[start_position.1][start_position.0-1], &Direction::West) {
        start_position.0 -= 1;
        start_travel(map, start_position, Direction::West, &mut path);
        start_position.0 += 1;
    }
    if start_position.0 < map[start_position.1].len() - 1 && can_enter(map[start_position.1][start_position.0+1], &Direction::East) {
        start_position.0 += 1;
        start_travel(map, start_position, Direction::East, &mut path);
        start_position.0 -= 1;
    }
    path.values().max().unwrap_or(&0).to_owned()
}

fn get_next_direction(position: char, current_direction: Direction) -> Direction {
    match (position, current_direction) {
        ('|', Direction::North) | ('L', Direction::West)  | ('J', Direction::East) => Direction::North,
        ('F', Direction::North) | ('L', Direction::South) | ('-', Direction::East) => Direction::East,
        ('7', Direction::North) | ('J', Direction::South) | ('-', Direction::West) => Direction::West,
        ('|', Direction::South) | ('F', Direction::West)  | ('7', Direction::East) => Direction::South,
        _ => unreachable!()

    }
}

fn start_travel(map: &Vec<Vec<char>>, start_position: (usize, usize), direction: Direction, path: &mut HashMap<(usize, usize), usize>) {
    let mut steps = 0;
    let (mut x, mut y) = start_position;
    let mut direction = direction;

    loop {
        steps += 1;
        if let Some(val) = path.get(&(x, y)) {
            if val < &steps {
                return
            }
        }
        path.insert((x, y), steps);
        direction = get_next_direction(map[y][x], direction);
        match direction {
            Direction::North => {
                if y > 0 && can_enter(map[y-1][x], &direction) {
                    y -= 1;
                } else {
                    return
                }
            }
            Direction::South => {
                if y < map.len() - 1 && can_enter(map[y+1][x], &direction) {
                    y += 1;
                } else {
                    return
                }
            }
            Direction::East => {
                if x < map[y].len() - 1 && can_enter(map[y][x+1], &direction) {
                    x += 1;
                } else {
                    return
                }
            }
            Direction::West => {
                if x > 0 && can_enter(map[y][x-1], &direction) {
                    x -= 1;
                } else {
                    return
                }
            }
        }
    }
}

fn travel_loop(map: &Vec<Vec<char>>, start_position: (usize, usize), direction: Direction, path: &mut HashMap<(usize, usize), usize>) -> bool {
    let (mut x, mut y) = start_position;
    let mut direction = direction;

    loop {
        if path.get(&(x, y)).is_some() {
            return false
        }
        path.insert((x, y), 1);
        direction = get_next_direction(map[y][x], direction);
        match direction {
            Direction::North => {
                if y > 0 && can_enter(map[y-1][x], &direction) {
                    y -= 1;
                } else {
                    if map[y-1][x] == 'S' {
                        return true
                    }
                    return false
                }
            }
            Direction::South => {
                if y < map.len() - 1 && can_enter(map[y+1][x], &direction) {
                    y += 1;
                } else {
                    if map[y+1][x] == 'S' {
                        return true
                    }
                    return false
                }
            }
            Direction::East => {
                if x < map[y].len() - 1 && can_enter(map[y][x+1], &direction) {
                    x += 1;
                } else {
                    if map[y][x+1] == 'S' {
                        return true
                    }
                    return false
                }
            }
            Direction::West => {
                if x > 0 && can_enter(map[y][x-1], &direction) {
                    x -= 1;
                } else {
                    if map[y][x-1] == 'S' {
                        return true
                    }
                    return false
                }
            }
        }
    }
}

fn get_possible_places(map: &Vec<Vec<char>>) -> usize {
    let mut start_position: (usize, usize) = (0, 0);

    'outer: for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.eq(&'S') {
                start_position = (x, y);
                break 'outer;
            }
        }
    }

    let mut path: HashMap<(usize, usize), usize> = HashMap::new();

    if start_position.1 > 0 && can_enter(map[start_position.1-1][start_position.0], &Direction::North) {
        start_position.1 -= 1;
        start_travel(map, start_position, Direction::North, &mut path);
        start_position.1 += 1;
    }
    if start_position.1 < map.len() - 1 && can_enter(map[start_position.1+1][start_position.0], &Direction::South) {
        start_position.1 += 1;
        start_travel(map, start_position, Direction::South, &mut path);
        start_position.1 -= 1;
    }
    if start_position.0 > 0 && can_enter(map[start_position.1][start_position.0-1], &Direction::West) {
        start_position.0 -= 1;
        start_travel(map, start_position, Direction::West, &mut path);
        start_position.0 += 1;
    }
    if start_position.0 < map[start_position.1].len() - 1 && can_enter(map[start_position.1][start_position.0+1], &Direction::East) {
        start_position.0 += 1;
        start_travel(map, start_position, Direction::East, &mut path);
        start_position.0 -= 1;
    }
    
    let bounding_box = get_bounding_box(&path);

    let mut inside = 0;
    for i in bounding_box.1+1..=bounding_box.3-1 {
        for j in bounding_box.0+1..=bounding_box.2-1 {
            if !path.contains_key(&(j, i)) && map[i][j] != 'S' {
                let mut passes = 0;
                let mut last_edge = '\0';
                for k in j..=bounding_box.2 {
                    if !path.contains_key(&(k, i)) {
                        continue;
                    }
                    match (last_edge, map[i][k]) {
                        (_, '|') => passes += 1,
                        (_, 'F' | 'L') => {
                            passes += 1;
                            last_edge = map[i][k];
                        }
                        ('F', '7') | ('L', 'J') => {
                            passes -= 1;
                            last_edge = '\0';
                        }
                        ('F', 'J') | ('L', '7') => {
                            last_edge = '\0';
                        }
                        _ => {}
                    }
                }
                if passes % 2 == 1 {
                    inside += 1;
                }
            }
        }
    }

    inside
}

fn get_bounding_box(path: &HashMap<(usize, usize), usize>) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in path.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    (min_x, min_y, max_x, max_y)
    
}

fn main() {
    let data = include_str!("../input.txt");
    let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
    println!("Result part 1: {}", get_furthest_point(&map));
    println!("Result part 2: {}", get_possible_places(&map));
}

#[cfg(test)]
mod tests {
    use super::{get_furthest_point, get_possible_places};

    #[test]
    fn test_get_furthest_point1() {
        let data = include_str!("../test1.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
        assert_eq!(get_furthest_point(&map), 4);
    }

    #[test]
    fn test_get_furthest_point2() {
        let data = include_str!("../test2.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
        assert_eq!(get_furthest_point(&map), 8);
    }

    #[test]
    fn test_get_possible_places1() {
        let data = include_str!("../test3.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
        assert_eq!(get_possible_places(&map), 4);
    }

    #[test]
    fn test_get_possible_places2() {
        let data = include_str!("../test4.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
        assert_eq!(get_possible_places(&map), 10);
    }

    
    #[test]
    fn test_get_possible_places3() {
        let data = include_str!("../test5.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.trim().chars().collect() }).collect();
        assert_eq!(get_possible_places(&map), 8);
    }
}