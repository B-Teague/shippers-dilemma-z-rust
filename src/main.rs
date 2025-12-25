use self::Direction::*;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
enum Direction {
    ForwardUp,
    ForwardRight,
    ForwardDown,
    ForwardLeft,
    RightUp,
    RightBack,
    RightDown,
    RightForward,
    BackUp,
    BackLeft,
    BackDown,
    BackRight,
    LeftUp,
    LeftForward,
    LeftDown,
    LeftBack,
    UpBack,
    UpRight,
    UpForward,
    UpLeft,
    DownForward,
    DownRight,
    DownBack,
    DownLeft,
}

impl Direction {
    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 24] = [
            ForwardUp,
            ForwardRight,
            ForwardDown,
            ForwardLeft,
            RightUp,
            RightBack,
            RightDown,
            RightForward,
            BackUp,
            BackLeft,
            BackDown,
            BackRight,
            LeftUp,
            LeftForward,
            LeftDown,
            LeftBack,
            UpBack,
            UpRight,
            UpForward,
            UpLeft,
            DownForward,
            DownRight,
            DownBack,
            DownLeft,
        ];
        DIRECTIONS.iter()
    }

    fn get_node_offsets(&self) -> [(i8, i8, i8); 5] {
        match self {
            //Right handed coordinate system, Y is up, negative Z is forward
            Direction::ForwardUp => [(0, 0, 0), (0, 0, -1), (0, 0, -2), (0, 1, -2), (0, 1, -3)],
            Direction::ForwardRight => [(0, 0, 0), (0, 0, -1), (0, 0, -2), (1, 0, -2), (1, 0, -3)],
            Direction::ForwardDown => [(0, 0, 0), (0, 0, -1), (0, 0, -2), (0, -1, -2), (0, -1, -3)],
            Direction::ForwardLeft => [(0, 0, 0), (0, 0, -1), (0, 0, -2), (-1, 0, -2), (-1, 0, -3)],

            Direction::RightUp => [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0), (3, 1, 0)],
            Direction::RightBack => [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 0, 1), (3, 0, 1)],
            Direction::RightDown => [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0), (3, -1, 0)],
            Direction::RightForward => [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 0, -1), (3, 0, -1)],

            Direction::BackUp => [(0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 1, 2), (0, 1, 3)],
            Direction::BackLeft => [(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 2), (-1, 0, 3)],
            Direction::BackDown => [(0, 0, 0), (0, 0, 1), (0, 0, 2), (0, -1, 2), (0, -1, 3)],
            Direction::BackRight => [(0, 0, 0), (0, 0, 1), (0, 0, 2), (1, 0, 2), (1, 0, 3)],

            Direction::LeftUp => [(0, 0, 0), (-1, 0, 0), (-2, 0, 0), (-2, 1, 0), (-3, 1, 0)],
            Direction::LeftForward => [(0, 0, 0), (-1, 0, 0), (-2, 0, 0), (-2, 0, -1), (-3, 0, -1)],
            Direction::LeftDown => [(0, 0, 0), (-1, 0, 0), (-2, 0, 0), (-2, -1, 0), (-3, -1, 0)],
            Direction::LeftBack => [(0, 0, 0), (-1, 0, 0), (-2, 0, 0), (-2, 0, 1), (-3, 0, 1)],

            Direction::UpBack => [(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 2, 1), (0, 3, 1)],
            Direction::UpRight => [(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 2, 0), (1, 3, 0)],
            Direction::UpForward => [(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 2, -1), (0, 3, -1)],
            Direction::UpLeft => [(0, 0, 0), (0, 1, 0), (0, 2, 0), (-1, 2, 0), (-1, 3, 0)],

            Direction::DownForward => [(0, 0, 0), (0, -1, 0), (0, -2, 0), (0, -2, -1), (0, -3, -1)],
            Direction::DownRight => [(0, 0, 0), (0, -1, 0), (0, -2, 0), (1, -2, 0), (1, -3, 0)],
            Direction::DownBack => [(0, 0, 0), (0, -1, 0), (0, -2, 0), (0, -2, 1), (0, -3, 1)],
            Direction::DownLeft => [(0, 0, 0), (0, -1, 0), (0, -2, 0), (-1, -2, 0), (-1, -3, 0)],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node(i8, i8, i8);

trait IsInBounds {
    fn is_in_bounds(&self) -> bool;
}

impl Node {
    fn new() -> Self {
        Self(0, 0, 0)
    }

    fn set_position(&mut self, x: i8, y: i8, z: i8) {
        *self = Node(x, y, z);
    }
}

impl IsInBounds for Node {
    fn is_in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 <= 4 && self.1 >= 0 && self.1 <= 4 && self.2 >= 0 && self.2 <= 4
    }
}

#[derive(Clone, Copy)]
struct Piece {
    nodes: [Node; 5],
    direction: Direction,
}

impl Piece {
    fn new() -> Self {
        Self {
            nodes: [Node::new(); 5],
            direction: Direction::ForwardUp,
        }
    }

    fn set_position(&mut self, x: i8, y: i8, z: i8) {
        self.nodes[0] = Node(x, y, z);
        self.update_nodes();
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.update_nodes();
    }

    fn update_nodes(&mut self) {
        for i in 1..=4 {
            let root_node = self.nodes[0];
            let offsets = self.direction.get_node_offsets();
            self.nodes[i].set_position(
                root_node.0 + offsets[i].0,
                root_node.1 + offsets[i].1,
                root_node.2 + offsets[i].2,
            );
        }
    }

    fn overlaps(&self, piece: Piece) -> bool {
        for n in self.nodes {
            for m in piece.nodes {
                if n == m {
                    return true;
                }
            }
        }
        false
    }
}

impl IsInBounds for Piece {
    fn is_in_bounds(&self) -> bool {
        for n in self.nodes {
            if !n.is_in_bounds() {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy)]
struct CubeNode {
    position: Node,
    position_heuristic: u8,
    direction_heuristic: u8,
    available_directions: [Option<Direction>; 12], //Using array instead of Vec to keep data on the stack
    is_occupied: bool,
    occupied_letter: char,
}

impl CubeNode {
    fn new() -> Self {
        Self {
            position: Node(0, 0, 0),
            position_heuristic: 0,
            direction_heuristic: 0,
            available_directions: [None; 12],
            is_occupied: false,
            occupied_letter: ' ',
        }
    }

    fn set_position(&mut self, x: i8, y: i8, z: i8) {
        self.position = Node(x, y, z);
    }
}

type CubeMatrix = [[[CubeNode; 5]; 5]; 5];

#[derive(Debug)]
struct Cube {
    cube_nodes: CubeMatrix,
    current_letter: char,
}

impl Cube {
    fn new() -> Self {
        let cube_nodes = Cube::initialize_heuristics();
        Self {
            cube_nodes,
            current_letter: 'A',
        }
    }

    fn initialize_heuristics() -> CubeMatrix {
        let mut cube_matrix = [[[CubeNode::new(); 5]; 5]; 5];
        let mut piece = Piece::new();
        for x in 0..=4 {
            for y in 0..=4 {
                for z in 0..=4 {
                    cube_matrix[x][y][z].set_position(x as i8, y as i8, z as i8);
                    piece.set_position(x as i8, y as i8, z as i8);
                    let mut available_index = 0;

                    for d in Direction::iterator() {
                        piece.set_direction(*d);

                        if piece.is_in_bounds() {
                            cube_matrix[x][y][z].direction_heuristic += 1;

                            for n in piece.nodes {
                                cube_matrix[n.0 as usize][n.1 as usize][n.2 as usize]
                                    .position_heuristic += 1;
                            }

                            cube_matrix[x][y][z].available_directions[available_index] = Some(*d);
                            available_index += 1;
                        }
                    }
                }
            }
        }

        cube_matrix
    }

    fn print_direction_heuristic(&self) -> String {
        let mut result: Vec<String> = vec![];
        for x in 0..=4 {
            for y in 0..=4 {
                let mut direction_heuristics: [u8; 5] = [0; 5];
                for z in 0..=4 {
                    direction_heuristics[z] = self.cube_nodes[x][y][z].direction_heuristic;
                }
                result.push(format!("{:?} ", direction_heuristics));
            }
            result.push(String::from('\n'));
        }
        result.join(&String::from(' '))
    }

    fn print_position_heuristic(&self) -> String {
        let mut result: Vec<String> = vec![];
        for x in 0..=4 {
            for y in 0..=4 {
                let mut position_heuristics: [u8; 5] = [0; 5];
                for z in 0..=4 {
                    position_heuristics[z] = self.cube_nodes[x][y][z].position_heuristic;
                }
                result.push(format!("{:?} ", position_heuristics));
            }
            result.push(String::from('\n'));
        }
        result.join(&String::from(' '))
    }

    fn print_puzzle_occupied(&self) -> String {
        let mut result: Vec<String> = vec![];
        for x in 0..=4 {
            for y in 0..=4 {
                let mut occupied: [char; 5] = ['*'; 5];
                for z in 0..=4 {
                    occupied[z] = self.cube_nodes[x][y][z].occupied_letter;
                }
                result.push(format!("{:?} ", occupied));
            }
            result.push(String::from('\n'));
        }
        result.join(&String::from(' '))
    }

    fn place_piece(&mut self, piece: Piece) {
        //set all nodes piece overlaps to occupied
        for n in piece.nodes {
            self.cube_nodes[n.0 as usize][n.1 as usize][n.2 as usize].is_occupied = true;
            self.cube_nodes[n.0 as usize][n.1 as usize][n.2 as usize].occupied_letter =
                self.current_letter;
        }

        self.current_letter = ((self.current_letter as u8) + 1) as char;

        //Update heuristics
        let mut temp_piece = Piece::new();
        for x in 0..=4 {
            for y in 0..=4 {
                for z in 0..=4 {
                    temp_piece.set_position(x, y, z);

                    for d_index in 0..12 {
                        let current_dir = self.cube_nodes[x as usize][y as usize][z as usize]
                            .available_directions[d_index];
                        match current_dir {
                            Some(dir) => {
                                temp_piece.set_direction(dir);
                                if temp_piece.overlaps(piece) {
                                    let position = temp_piece.nodes[0];
                                    self.cube_nodes[position.0 as usize][position.1 as usize]
                                        [position.2 as usize]
                                        .direction_heuristic -= 1;

                                    self.cube_nodes[position.0 as usize][position.1 as usize]
                                        [position.2 as usize]
                                        .available_directions[d_index] = None;

                                    for n in temp_piece.nodes {
                                        self.cube_nodes[n.0 as usize][n.1 as usize]
                                            [n.2 as usize]
                                            .position_heuristic -= 1;
                                    }
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }

    fn find_optimal_piece(&self) -> Piece {
        let mut min_direction = 99;
        let mut min_position: u8;
        let mut optimal_position = Node(0, 0, 0);
        let mut optimal_direction = Direction::ForwardUp;
        let mut temp_piece = Piece::new();

        for x in 0..=4 {
            for y in 0..=4 {
                for z in 0..=4 {
                    let direction_heuristic = self.cube_nodes[x][y][z].direction_heuristic;
                    if direction_heuristic > 0 && direction_heuristic < min_direction {
                        optimal_position = Node(x as i8, y as i8, z as i8);
                        min_direction = self.cube_nodes[x][y][z].direction_heuristic;
                        temp_piece.set_position(
                            optimal_position.0,
                            optimal_position.1,
                            optimal_position.2,
                        );

                        min_position = 99; //Reset min position each time a new min direction is found.

                        //For each direction, find the minimal position node for each node in the piece
                        for d in self.cube_nodes[x][y][z].available_directions {
                            match d {
                                Some(dir) => {
                                    temp_piece.set_direction(dir);

                                    for n in temp_piece.nodes {
                                        let position_heuristic = self.cube_nodes[n.0 as usize]
                                            [n.1 as usize]
                                            [n.2 as usize]
                                            .position_heuristic;
                                        if position_heuristic > 0
                                            && position_heuristic < min_position
                                        {
                                            optimal_direction = dir;
                                            min_position = self.cube_nodes[n.0 as usize]
                                                [n.1 as usize]
                                                [n.2 as usize]
                                                .position_heuristic;
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
            }
        }

        let mut optimal_piece = Piece::new();
        optimal_piece.set_position(optimal_position.0, optimal_position.1, optimal_position.2);
        optimal_piece.set_direction(optimal_direction);
        optimal_piece
    }
}

fn main() {
    let mut puzzle = Cube::new();
    let mut next_piece: Piece;
    for _ in 1..=4 {
        next_piece = puzzle.find_optimal_piece();
        puzzle.place_piece(next_piece);
    }

    println!("{}", puzzle.print_direction_heuristic());
    println!("{}", puzzle.print_position_heuristic());
    println!("{}", puzzle.print_puzzle_occupied());
}
