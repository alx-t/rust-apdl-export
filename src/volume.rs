use point::Point;

pub struct Volume {
    pub p0: Point,
    pub p1: Point
}

impl Volume {
    pub fn new(p0: Point, p1: Point) -> Volume {
        Volume {
            p0: p0,
            p1: p1
        }
    }

    fn dir(&self) -> Point {
        Point::new(
            if self.x() >= 0 { 1 } else { -1 },
            if self.y() >= 0 { 1 } else { -1 },
            if self.z() >= 0 { 1 } else { -1 },
        )
    }

    fn x(&self) -> i32 { self.p1.x - self.p0.x }
    fn y(&self) -> i32 { self.p1.y - self.p0.y }
    fn z(&self) -> i32 { self.p1.z - self.p0.z }

    fn size(&self) -> Point {
        Point::new(self.x().abs(), self.y().abs(), self.z().abs())
    }
}

pub struct Filler {
    pub volume: Volume,
    pub filler: Volume,
    pub rotation: Point
}

impl Filler {
    pub fn new(volume: Volume, filler: Volume, rotation: Point) -> Filler {
        Filler {
            volume: volume,
            filler: filler,
            rotation: Filler::normalize(rotation)
        }
    }

    pub fn result(&self) -> Vec<Volume> {
        let sx = self.step_x() * self.volume.dir().x;
        let sy = self.step_y() * self.volume.dir().y;
        let sz = self.step_z() * self.volume.dir().z;

        let mut res: Vec<Volume> = vec![];

        for x_i in 0..self.x_times() {
            for y_i in 0..self.y_times() {
                for z_i in 0..self.z_times() {
                    res.push(Volume::new(
                        Point::new(self.volume.p0.x + x_i * sx,
                                    self.volume.p0.y + y_i * sy,
                                    self.volume.p0.z + z_i * sz),
                        Point::new(self.volume.p0.x + x_i * sx + sx,
                                    self.volume.p0.y + y_i * sy + sy,
                                    self.volume.p0.z + z_i * sz + sz)
                    ));
                }
            }
        }
        res
    }

    fn x(&self) -> i32 { self.volume.size().x }
    fn y(&self) -> i32 { self.volume.size().y }
    fn z(&self) -> i32 { self.volume.size().z }

    fn step_x(&self) -> i32 {
        match self.rotation {
            Point { x: 90, y: 90, z: 90 }
                | Point { x: 90, y: 0, z: 90 }
                | Point { x: 0, y: 90, z: 0 } => return self.filler.size().z,
            Point { x: 90, y: 90, z: 0 }
                | Point { x: 0, y: 90, z: 90 }
                | Point { x: 0, y: 0, z: 90 } => return self.filler.size().y,
            _ => return self.filler.size().x,
        }
    }

    fn step_y(&self) -> i32 {
        match self.rotation {
            Point { x: 90, y: 90, z: 0 }
                | Point { x: 0, y: 90, z: 90 }
                | Point { x: 90, y: 0, z: 0 } => return self.filler.size().z,
            Point { x: 90, y: 0, z: 90 }
                | Point { x: 0, y: 0, z: 90 } => return self.filler.size().x,
            _ => return self.filler.size().y,
        }
    }

    fn step_z(&self) -> i32 {
        match self.rotation {
            Point { x: 90, y: 90, z: 90 }
                | Point { x: 90, y: 90, z: 0 }
                | Point { x: 0, y: 0, z: 90 }
                | Point { x: 0, y: 90, z: 0 } => return self.filler.size().x,
            Point { x: 90, y: 0, z: 90 }
                | Point { x: 90, y: 0, z: 0 } => return self.filler.size().y,
            _ => return self.filler.size().z,
        }
    }

    fn x_times(&self) -> i32 { self.x() / self.step_x() }
    fn y_times(&self) -> i32 { self.y() / self.step_y() }
    fn z_times(&self) -> i32 { self.z() / self.step_z() }

    fn normalize(rotation: Point) -> Point {
        Point::new(
            Filler::normalize_direction(rotation.x),
            Filler::normalize_direction(rotation.y),
            Filler::normalize_direction(rotation.z)
        )
    }

    fn normalize_direction(r: i32) -> i32 {
        if ((r / 90) as i32) % 2 == 0 { return 0 }
        if ((r / 90) as i32) % 2 != 0 { return 90 }
        0
    }
}
