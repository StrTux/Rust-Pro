enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction  = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction)
}

fn move_around(direction: Direction) {
    match (direction) {
        Direction::North => println!("Moving North"),
        Direction::East => println!("Moving East"),
        Direction::South => println!("Moving South"),
        Direction::West => println!("Moving West"),
    }
}

// saame inn  c++  
// #include <iostream>
// using namespace std;

// enum Direction {
//     North,
//     East,
//     South,
//     West
// };

// void move_around(Direction direction) {
//     switch (direction) {
//         case North:
//             cout << "Moving North" << endl;
//             break;
//         case East:
//             cout << "Moving East" << endl;
//             break;
//         case South:
//             cout << "Moving South" << endl;
//             break;
//         case West:
//             cout << "Moving West" << endl;
//             break;
//         default:
//             cout << "Unknown direction" << endl;
//             break;
//     }
// }

// int main() {
//     Direction my_direction = North;
//     Direction new_direction = my_direction;
//     move_around(new_direction);
//     return 0;
// }
