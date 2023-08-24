use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

#[pyclass]
#[derive(Clone)]

// Declaration of struct Grid:
//     - cell_type can either be: wall, room, or hallway
//     - room_id keeps an ID on each room
//     - is merged checks to see if a cell is merged
//     - room_size keeps track of how large a room is

pub struct Grid {
    cell_type: String,
    room_id: usize,
    is_merged: bool,
    room_size: usize,
}



#[pymethods]
impl Grid {
    #[getter]
    fn cell_type(&self) -> PyResult<String> {
        Ok(self.cell_type.clone())
    }
}

// Constant created in case non-overlapping rooms can not be created.
const MAX_ATTEMPTS: usize = 1000;

// Constant for number of rooms user wants to create.
const NUMBER_OF_ROOMS: usize = 10;

// Constant for the width of hallways. 
const HALLWAY_WIDTH: usize = 2;


//----- GENERATE WALLS OF SIMULATION -----//

// - Grid_size is the size of the grid(the representation of the pygame simulation)
// - Room_size_factor determines the size of the rooms. 
// - If it is closer to one, then the rooms are larger
#[pyfunction]
pub fn generate_walls(grid_size: usize, room_size_factor: f64) -> PyResult<Vec<Vec<Grid>>> {
    let mut grid: Vec<Vec<Grid>> = vec![vec![Grid {
        cell_type: "wall".to_string(),
        room_id: 0,
        is_merged: false,
        room_size: 0,
    }; grid_size]; grid_size];

    let mut rng = rand::thread_rng();

    let min_room_size = (grid_size as f64 * 0.1 * room_size_factor).max(3.0) as usize;
    let max_room_size = (grid_size as f64 * 0.5 * room_size_factor).max(min_room_size as f64) as usize;
    let min_distance = 5; // Minimum distance between rooms
    println!("Min room size: {}", min_room_size);
    println!("Max room size: {}", max_room_size);



    for room_id in 0..NUMBER_OF_ROOMS {
        let mut attempts = 0;
        loop {
            attempts += 1; 

            // These variables creates the width and height of the rooms
            let room_width = rng.gen_range(min_room_size..max_room_size);
            let room_height = rng.gen_range(min_room_size..max_room_size);

            // These variables are used to check if the code can avoid overlapping rooms 
            let start_x = rng.gen_range(0..grid_size - room_width);
            let start_y = rng.gen_range(0..grid_size - room_height);

            let mut overlap = false;
            let mut too_close = false;

            // Check for overlap
            for y in start_y..start_y + room_height {
                for x in start_x..start_x + room_width {
                    if grid[y][x].cell_type == "room" {
                        overlap = true;
                        break;
                    }
                }
                if overlap {
                    break;
                }
            }

            // Check the distance to existing rooms
            if !overlap {
                for other_y in 0..grid_size {
                    for other_x in 0..grid_size {
                        if grid[other_y][other_x].cell_type == "room" {
                            let distance = (((other_x as isize - start_x as isize).pow(2) +
                                (other_y as isize - start_y as isize).pow(2)) as f64).sqrt() as usize;

                            if distance < min_distance {
                                too_close = true;
                                break;
                            }
                        }
                    }
                    if too_close {
                        break;
                    }
                }
            }

            if !overlap && !too_close || attempts >= MAX_ATTEMPTS {
                if !overlap && !too_close {
                    for y in start_y..start_y + room_height {
                        for x in start_x..start_x + room_width {
                            grid[y][x] = Grid {
                                cell_type: "room".to_string(),
                                room_id,
                                is_merged: false,
                                room_size: room_width * room_height,
                            };
                        }
                    }
                    break;
                }
            }
        }
    }

    Ok(grid)
}


#[pymodule]
pub fn my_rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_walls, m)?)?;
    m.add_function(wrap_pyfunction!(generate_hallways, m)?)?;

    Ok(())
}

//----- GENERATE HALLWAYS FUNCTION -----//

#[pyfunction]
pub fn generate_hallways(grid: Vec<Vec<Grid>>, hallway_factor: f64) -> PyResult<Vec<Vec<Grid>>> {
    let mut rng = rand::thread_rng();
    let mut new_grid = grid.clone();
    let mut centers = Vec::new();
    let hallway_width = HALLWAY_WIDTH;

    // Calculate the center of the rooms
    for room_id in 0..NUMBER_OF_ROOMS {
        let mut x_sum = 0;
        let mut y_sum = 0;
        let mut count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x].room_id == room_id {
                    x_sum += x;
                    y_sum += y;
                    count += 1;
                }
            }
        }
        if count > 0 {
            centers.push((x_sum / count, y_sum / count, room_id));
        }
    }

    // Create a list of edges with distances
    let mut edges = Vec::new();
    for i in 0..centers.len() {
        for j in i + 1..centers.len() {
            let distance = (((centers[i].0 as isize - centers[j].0 as isize).pow(2) +
                (centers[i].1 as isize - centers[j].1 as isize).pow(2)) as f64)
                .sqrt() as usize;
            edges.push((i, j, distance));
        }
    }

    // Sort edges by distance
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    // Use Kruskal's algorithm to create a minimum spanning tree
    let mut sets: Vec<usize> = (0..centers.len()).collect();
    for (i, j, _) in &edges {
        if sets[*i] != sets[*j] { // Check if rooms are connected
            let set_i = sets[*i];
            let set_j = sets[*j];
            for set in sets.iter_mut() {
                if *set == set_j {
                    *set = set_i;
                }
            }
            connect_rooms(&mut new_grid, centers[*i], centers[*j], hallway_width);
        }
    }

    // Add additional connections based on the hallway factor
    for (i, j, _) in edges {
        if rng.gen::<f64>() < hallway_factor {
            connect_rooms(&mut new_grid, centers[i], centers[j], hallway_width);
        }
    }

    Ok(new_grid)
}

//----- CONNECT ROOMS FUNCTION -----//

fn connect_rooms(grid: &mut Vec<Vec<Grid>>, room1: (usize, usize, usize), room2: (usize, usize, usize), hallway_width: usize) {
    let (x1, y1, _) = room1;
    let (x2, y2, _) = room2;
    let half_width = hallway_width / 2;

    // Draw horizontal part of the hallway
    let (hx_start, hx_end) = (x1.min(x2), x1.max(x2));
    let (hy_start, hy_end) = (y1.min(y2), y1.max(y2));
    
    // Draw horizontal hallway
    for x in hx_start..=hx_end {
        for offset in 0..hallway_width {
            let y = y1.saturating_sub(offset);
            if y < grid.len() && x < grid[0].len() {
                grid[y][x].cell_type = "hallway".to_string();
            }
        }
    }

    // Draw vertical hallway
    for y in hy_start..=hy_end {
        for offset in 0..hallway_width {
            let x = x2.saturating_sub(offset);
            if y < grid.len() && x < grid[0].len() {
                grid[y][x].cell_type = "hallway".to_string();
            }
        }
    }
}

