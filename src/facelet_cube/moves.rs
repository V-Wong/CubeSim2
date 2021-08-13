use std::collections::HashMap;

use crate::generic_cube::{Cube, Move, MoveVariant};
use crate::geometric_cube::{GeoCube, Sticker, cube3};

pub fn from_geometric_move(mv: Move) -> Vec<(i32, i32)> {
    let index_map: HashMap<(i32, i32, i32), i32> = create_index_conversion_map();

    cube3().apply_move(mv).0
           .iter()
           .map(|s|
                match (index_map.get(&(s.destination.x, s.destination.y, s.destination.z)),
                       index_map.get(&(s.position.x, s.position.y, s.position.z))
                ) {
                    (Some(x), Some(y)) => return (x.clone(), y.clone()),
                    (_, _) => panic!()
                }                 
           )
           .filter(|x| x.0 != x.1)
           .collect()
}

fn create_index_conversion_map() -> HashMap<(i32, i32, i32), i32> {
    use Move::*;
    use MoveVariant::*;

    let mut map = HashMap::new();

    let face_rotating_moves = vec![
        vec![],
        vec![X(Inverse), Y(Inverse)],
        vec![X(Inverse)],
        vec![X(Double)],
        vec![X(Inverse), Y(Standard)],
        vec![X(Inverse), Y(Double)]
    ];

    let mut idx = 0;
    for rotation in face_rotating_moves {
        for z in [-2, 0, 2] {
            for x in [-2, 0, 2] {
                let pos = GeoCube(vec![Sticker::new(x, 3, z)])
                                .apply_moves(rotation.clone()).0[0].position;
                map.insert((pos.x, pos.y, pos.z), idx);
                idx += 1;
            }
        }
    }

    map
}
