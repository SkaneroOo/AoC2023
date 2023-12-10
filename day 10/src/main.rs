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

fn main() {
    let data = include_str!("../input.txt");
    let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.chars().collect() }).collect();
    println!("Result: {}", get_furthest_point(&map));
}

#[cfg(test)]
mod tests {
    use super::get_furthest_point;

    #[test]
    fn test_get_furthest_point1() {
        let data = include_str!("../test1.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.chars().collect() }).collect();
        assert_eq!(get_furthest_point(&map), 4);
    }

    #[test]
    fn test_get_furthest_point2() {
        let data = include_str!("../test2.txt");
        let map: Vec<Vec<char>> = data.split('\n').collect::<Vec<&str>>().iter().map(|x| { x.chars().collect() }).collect();
        assert_eq!(get_furthest_point(&map), 8);
    }
}