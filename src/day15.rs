const OFF: i32 = 511300;
const WIDTH: usize = 8000000;
const ROW_CHECK: i32 = 200000;

// Part 2
const P2_MAX: i32 = 400000; // val + 1

#[derive(Debug)]
struct Pos(i32, i32); // row, col

#[derive(Debug)]
struct Sensor {
  pos: Pos,
  beacon: Pos,
  dist: i32,
}

#[derive(Default, Copy, Clone, Debug)]
struct Cell {
  filled: bool,
  beacon: bool,
  //sensor: bool,
}

pub fn day15() {
  // Parse
  let mut sensors: Vec<Sensor> = Vec::new();

  let inp = include_str!("day15.txt").replace("x=", "").replace("y=", "");
  for line in inp.split("\n") {
    let parts = line.split(": closest beacon is at ").collect::<Vec<_>>();
    let sposvals = parts[0].replace("Sensor at ", "").split(", ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let bposvals = parts[1].split(", ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    sensors.push(Sensor {
      pos: Pos(sposvals[1], sposvals[0]),
      beacon: Pos(bposvals[1], bposvals[0]),
      dist: (sposvals[1] - bposvals[1]).abs() + (sposvals[0] - bposvals[0]).abs(),
    });
  }

  // Fill row
  let mut row = vec![Cell::default(); WIDTH];
  for sensor in sensors.iter() {
    let dist = (sensor.pos.0 - sensor.beacon.0).abs() + (sensor.pos.1 - sensor.beacon.1).abs();

    // Check if sensor or beacon are in row
    if sensor.pos.0 == ROW_CHECK {
      // Sensor
      //row[(sensor.pos.1 + OFF) as usize].sensor = true;
    } else if sensor.beacon.0 == ROW_CHECK {
      // Beacon
      row[(sensor.beacon.1 + OFF) as usize].beacon = true;
    }

    // Sensor overlaps
    if sensor.pos.0 - dist < ROW_CHECK && sensor.pos.0 + dist > ROW_CHECK {
      let rowdist = dist - (sensor.pos.0 - ROW_CHECK).abs();
      for i in (sensor.pos.1 + OFF) - rowdist..(sensor.pos.1 + OFF) + rowdist + 1 {
        if i < 0 {
          panic!("increase row offset by {}", i.abs());
        }
        row[i as usize].filled = true;
      }
    }
  }

  // Part 1
  println!("Part 1: {}", row.iter().filter(|x| x.filled && !x.beacon /* && !x.sensor */).count());

  // Part 2
  sensors.sort_by_key(|x| x.dist);
  for row in 0..P2_MAX {
    println!("Row: {}", row);
    for col in 0..P2_MAX {
      let mut filled = false;
      for sensor in sensors.iter() {
        if (row - sensor.pos.0).abs() + (col - sensor.pos.1).abs() <= sensor.dist {
          filled = true;
          break;
        }
      }
      if !filled {
        println!("Part 2: {}", col * P2_MAX + row);
        panic!("DONE");
      }
    }
  }
} 