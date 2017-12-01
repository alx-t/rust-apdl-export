use point::Point;
use volume::{Volume, Filler};
use apdl::{Element};

pub struct Block {
    pub coord_start: Point,
    pub coord_end: Point,
    pub elements: Vec<Element>
}

impl Block {
    pub fn new(coord_start: Point, coord_end: Point) -> Block {
        Block {
            coord_start: coord_start,
            coord_end: coord_end,
            elements: Block::create_elements(coord_start, coord_end)
        }
    }

    // TODO: Проверить создание и тп по блокам
    fn create_elements(coord_start: Point, coord_end: Point) -> Vec<Element> {
        let filler = Volume::new(Point::new(0,0,0), Point::new(25, 25, 25));

        let volumes = Filler::new(
            Volume::new(coord_start, coord_end),
            filler,
            Point::new(0, 0, 0)
        ).result();

        let mut result: Vec<Element> = Vec::new();

        for (pos, val) in volumes.iter().enumerate() {
            result.push(Element::new(pos + 1, val.p0, val.p1));
        }

        result
    }
}
