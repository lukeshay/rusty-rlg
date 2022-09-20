use rand::prelude::*;

const MAX_ROOMS: u8 = 8;
const MAX_ROOM_WIDTH: u8 = 9;
const MIN_ROOM_WIDTH: u8 = 4;
const MAX_ROOM_HEIGHT: u8 = 7;
const MIN_ROOM_HEIGHT: u8 = 3;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SpaceType {
    Unbreakable = b'#',
    Rock = b' ',
    Open = b'X',
}

impl Into<char> for SpaceType {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Clone, Copy)]
pub struct Space {
    space_type: SpaceType,
}

impl Space {
    pub fn new(space_type: SpaceType) -> Space {
        Space {
            space_type: space_type,
        }
    }

    pub fn character(&self) -> char {
        self.space_type.into()
    }
}

#[derive(Debug)]
struct Room(i32, i32, i32, i32);

impl Room {
    fn rand() -> Room {
        let mut rng = thread_rng();

        Room(rng.gen_range(1..(29 - MAX_ROOM_HEIGHT)) as i32, rng.gen_range(1..(79 - MAX_ROOM_WIDTH)) as i32, rng.gen_range(MIN_ROOM_HEIGHT..=MAX_ROOM_HEIGHT) as i32, rng.gen_range(MIN_ROOM_WIDTH..=MAX_ROOM_WIDTH) as i32)
    }

    fn intersects(&self, room: &Room) -> bool {
        let mut x_match = false;
        let mut y_match = false;

        for y1 in self.0..(self.0 + self.2 + 1) {
            for y2 in room.0..(room.0 + room.2 + 1) {
                y_match = y1 == y2;

                if y_match {
                    break;
                }
            }

            if y_match {
                break;
            }
        }

        for x1 in self.1..(self.1 + self.3 + 1) {
            for x2 in room.1..(room.1 + room.3 + 1) {
                x_match = x1 == x2;

                if x_match {
                    break;
                }
            }

            if x_match {
                break;
            }
        }

        return y_match && x_match;
    }
}

pub struct Dungeon {
    pub map: [[Space; 80]; 29],
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let mut map = [[Space::new(SpaceType::Rock); 80]; 29];

        for y in 0..29 {
            map[y][0] = Space::new(SpaceType::Unbreakable);
            map[y][79] = Space::new(SpaceType::Unbreakable);
        }

        for x in 0..80 {
            map[0][x] = Space::new(SpaceType::Unbreakable);
            map[28][x] = Space::new(SpaceType::Unbreakable);
        }

        let mut rooms: Vec<Room> = vec![];

        for _x in 0..5000 {
            let room = Room::rand();
            let intersecting_room = rooms.iter().find(|e| room.intersects(e));

            if intersecting_room.is_none() {
                rooms.push(room);
            }

            if rooms.len() == MAX_ROOMS as usize {
                break;
            }
        }

        tracing::debug!("Rooms: {:?}", rooms);

        for room in rooms.iter() {
            for y in room.0..(room.0 + room.2) {
                for x in room.1..(room.1 + room.3) {
                    map[y as usize][x as usize] = Space::new(SpaceType::Open);
                }
            }
        }


        Dungeon { map: map }
    }
}
