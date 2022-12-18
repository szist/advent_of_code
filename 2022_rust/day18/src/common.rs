pub const NEIGHBORS: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[derive(Clone, PartialEq, Eq)]
enum Vertex {
    LAVA,
    AIR,
    EXPOSED,
}

pub fn surface(cubes: &Vec<(usize, usize, usize)>, outer: bool) -> Option<usize> {
    let max_x = cubes.iter().map(|c| c.0).max()?;
    let max_y = cubes.iter().map(|c| c.1).max()?;
    let max_z = cubes.iter().map(|c| c.2).max()?;

    // create the 3d cube that contains this
    // with a padding, so we can fill it up
    const PADDING: usize = 1;
    let default_vertex = if outer { Vertex::AIR } else { Vertex::EXPOSED };
    // this structure ain't the most optimal, but at least it's easy to index
    // a co-located linearized structure would work better
    let mut map_3d =
        vec![
            vec![vec![default_vertex; max_z + 2 * PADDING + 1]; max_y + 2 * PADDING + 1];
            max_x + 2 * PADDING + 1
        ];

    // x,y,z coordinates go from 0..max_*
    for &(x, y, z) in cubes {
        map_3d[x + PADDING][y + PADDING][z + PADDING] = Vertex::LAVA;
    }

    if outer {
        // fill flood the map
        let seed = (
            map_3d.len() - 1,
            map_3d[0].len() - 1,
            map_3d[0][0].len() - 1,
        );
        let mut queue = vec![seed.clone()];
        while !queue.is_empty() {
            let v = queue.pop()?;
            map_3d[v.0][v.1][v.2] = Vertex::EXPOSED;
            for (dx, dy, dz) in NEIGHBORS {
                if (dx < 0 && v.0 == 0) || (dy < 0 && v.1 == 0) || (dz < 0 && v.2 == 0) {
                    continue;
                }
                let x = (v.0 as isize + dx) as usize;
                let y = (v.1 as isize + dy) as usize;
                let z = (v.2 as isize + dz) as usize;

                if x <= seed.0 && y <= seed.1 && z <= seed.2 && map_3d[x][y][z] == Vertex::AIR {
                    queue.push((x, y, z));
                }
            }
        }
    }

    let mut result = 0;

    for x in PADDING..=max_x + PADDING {
        for y in PADDING..=max_y + PADDING {
            for z in PADDING..=max_z + PADDING {
                if map_3d[x][y][z] != Vertex::LAVA {
                    continue;
                }
                let mut open_sides = 0;
                for (tx, ty, tz) in NEIGHBORS {
                    let tx = x as isize + tx;
                    let ty = y as isize + ty;
                    let tz = z as isize + tz;

                    // never will be out of bounds thanks to padding
                    if map_3d[tx as usize][ty as usize][tz as usize] == Vertex::EXPOSED {
                        open_sides += 1;
                    }
                }
                result += open_sides;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_trivial() {
        assert_eq!(surface(&vec![(1, 1, 1), (2, 1, 1)], true), Some(10));
    }
}
