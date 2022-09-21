use rand::prelude::*;

const MIN_ROOM_WIDTH: i32 = 6;
const MIN_ROOM_HEIGHT: i32 = 4;

const MIN_SECTION_HEIGHT: i32 = 6;
const MIN_SECTION_WIDTH: i32 = 8;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SpaceType {
    Rock = b'#',
    Open = b' ',
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
struct Coordinate {
    row: i32,
    column: i32,
}

#[derive(Clone, Copy, Debug)]
struct Section {
    row: i32,
    column: i32,
    height: i32,
    width: i32,
}

impl Section {
    fn new(row: i32, column: i32, height: i32, width: i32) -> Section {
        Section {
            row: row,
            column: column,
            height: height,
            width: width,
        }
    }

    fn top_right(&self) -> Coordinate {
        Coordinate {
            row: self.row,
            column: self.column + self.width,
        }
    }

    fn bottom_left(&self) -> Coordinate {
        Coordinate {
            row: self.row + self.height,
            column: self.column,
        }
    }

    fn generate_subsection(&self) -> Section {
        let mut rng = thread_rng();

        let min_row = self.row + 1;
        let max_row = self.row + self.height - 1 - MIN_ROOM_HEIGHT;
        let min_column = self.column + 1;
        let max_column = self.column + self.width - 1 - MIN_ROOM_WIDTH;

        tracing::debug!(
            "min_row: {}, max_row: {}, min_column: {}, max_column: {}",
            min_row,
            max_row,
            min_column,
            max_column
        );

        let row = rng.gen_range(min_row..=max_row);
        let column = rng.gen_range(min_column..=max_column);
        let height = rng.gen_range(MIN_ROOM_HEIGHT..=(self.row + self.height - row));
        let width = rng.gen_range(MIN_ROOM_WIDTH..=(self.column + self.width - column));

        Section::new(row, column, height, width)
    }
}

pub struct Dungeon {
    pub map: [[Space; 80]; 29],
}

fn calculate_split_point(start_point: i32, size: i32, min_size: i32) -> i32 {
    let mut rng = thread_rng();
    let midpoint = (size / 2) + start_point;
    let offset = ((size - 3) - (min_size * 2)) / 2;

    tracing::debug!("midpoint: {}, offset: {}", midpoint, offset);

    if offset == 0 {
        midpoint
    } else {
        midpoint + rng.gen_range((offset * -1)..=(offset))
    }
}

fn create_vertical_sections(from_section: Section) -> (Section, Section) {
    let column = calculate_split_point(from_section.column, from_section.width, MIN_SECTION_WIDTH);

    let first_section = Section::new(
        from_section.row,
        from_section.column,
        from_section.height,
        column - from_section.column,
    );
    let second_section = Section::new(
        from_section.row,
        column + 1,
        from_section.height,
        from_section.top_right().column - column - 1,
    );

    (first_section, second_section)
}

fn create_horizontal_sections(from_section: Section) -> (Section, Section) {
    let row = calculate_split_point(from_section.row, from_section.height, MIN_SECTION_HEIGHT);

    let first_section = Section::new(
        from_section.row,
        from_section.column,
        row - from_section.row,
        from_section.width,
    );

    let second_section = Section::new(
        row + 1,
        from_section.column,
        from_section.bottom_left().row - row - 1,
        from_section.width,
    );

    (first_section, second_section)
}

fn create_sections_rec(mut sections: Vec<Section>, from_section: Section) -> Vec<Section> {
    let cannot_split_horizontally = (from_section.height - 3) / 2 <= MIN_SECTION_HEIGHT;
    let cannot_split_vertically = (from_section.width - 3) / 2 <= MIN_SECTION_WIDTH;

    if cannot_split_horizontally && cannot_split_vertically {
        sections.push(from_section);

        return sections;
    }

    let (first_new_section, second_new_section) =
        if cannot_split_horizontally || from_section.width > from_section.height * 2 {
            create_vertical_sections(from_section)
        } else {
            create_horizontal_sections(from_section)
        };

    return create_sections_rec(
        create_sections_rec(sections, first_new_section),
        second_new_section,
    );
}

fn create_sections() -> Vec<Section> {
    let sections: Vec<Section> = vec![];

    let first_section = Section::new(0, 0, 28, 79);

    return create_sections_rec(sections, first_section);
}

impl Dungeon {
    pub fn new() -> Dungeon {
        let mut rng = thread_rng();
        let mut map = [[Space::new(SpaceType::Rock); 80]; 29];
        let sections = create_sections();

        tracing::debug!(
            "Number of sections: {}, sections: {:#?}",
            sections.len(),
            sections
        );

        for section in sections.iter() {
            if rng.gen::<f64>() > 0.80 {
                continue;
            }

            let room = section.generate_subsection();

            for y in room.row..(room.row + room.height) {
                for x in room.column..(room.column + room.width) {
                    if rng.gen::<f64>() > 0.95 {
                        continue;
                    }

                    map[y as usize][x as usize] = Space::new(SpaceType::Open);
                }
            }
        }

        Dungeon { map: map }
    }
}
