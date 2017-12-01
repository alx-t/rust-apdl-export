mod point;
mod volume;
mod block;
mod apdl;

extern crate time;

use time::PreciseTime;
use std::fs::OpenOptions;
use std::io::prelude::*;

use point::Point;
use block::Block;
use apdl::{Node, Element};

fn write_file(elements: Vec<Element>) {
    let file =
        OpenOptions::new()
        .write(true)
        .create(true)
        .open("export.cdb")
        .unwrap();
    let node = Node::new(&elements);
    nblock(&file, &node);
    eblock(&file, &elements, &node);
}

fn nblock(mut file: &std::fs::File, node: &Node) {
    println!("{:?}", node.map.len());
    writeln!(file, "/PREP7").expect("error");
    writeln!(file, "ET,        1,185").expect("error");
    writeln!(file, "NBLOCK,6,SOLID,{0: >10}{1: >10}", node.map.len(), node.map.len()).expect("error");
    writeln!(file, "(3i9,6e21.13e1)").expect("error");
    for nd in node.map.values() {
        writeln!(file, "{0: >9}{1: >9}{2: >9}{3:21.13E}{4:21.13E}{5:21.13E}",
            nd.num, 0, 0, nd.x as f32, nd.y as f32, nd.z as f32).expect("error");;
    }
    writeln!(file, "-1").expect("error");
}

fn eblock(mut file: &std::fs::File, elements: &Vec<Element>, node: &Node) {
    println!("{:?}", elements.len());
    writeln!(file, "EBLOCK,19,SOLID,{0: >10},{1: >10}", elements.len(), elements.len()).expect("error");
    writeln!(file, "(19i9)").expect("error");
    for (pos, elem) in elements.iter().enumerate() {
        let v = elem.nodes_nums(node);
        writeln!(file, "{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}",
            1, 1, 1, 1, 0, 0, 0, 0, 8, 0, pos + 1, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]).expect("error");;
    }
    writeln!(file, "-1").expect("error");
    writeln!(file, "FINISH").expect("error");
}

fn main() {
    let start = PreciseTime::now();

    let blocks = [
                    Block::new(Point::new(0, 0, 0), Point::new(100, 400, 400)),
                    Block::new(Point::new(100, 0, 0), Point::new(200, 400, 400)),
                    Block::new(Point::new(200, 0, 0), Point::new(300, 400, 400)),
                    Block::new(Point::new(300, 0, 0), Point::new(400, 400, 400)),
                    // Block::new(Point::new(400, 0, 0), Point::new(500, 400, 400)),
                    // Block::new(Point::new(500, 0, 0), Point::new(600, 400, 400)),
                    // Block::new(Point::new(600, 0, 0), Point::new(700, 400, 400)),
                    // Block::new(Point::new(700, 0, 0), Point::new(800, 400, 400)),
                    // Block::new(Point::new(800, 0, 0), Point::new(900, 400, 400)),
                    // Block::new(Point::new(900, 0, 0), Point::new(1000, 400, 400)),
                    // Block::new(Point::new(1000, 0, 0), Point::new(1100, 400, 400)),
                    // Block::new(Point::new(1100, 0, 0), Point::new(1200, 400, 400)),
                    // Block::new(Point::new(1200, 0, 0), Point::new(1300, 400, 400)),
                    // Block::new(Point::new(1300, 0, 0), Point::new(1400, 400, 400)),
                    // Block::new(Point::new(1400, 0, 0), Point::new(1500, 400, 400)),
                    // Block::new(Point::new(1500, 0, 0), Point::new(1600, 400, 400)),
                    // Block::new(Point::new(1600, 0, 0), Point::new(1700, 400, 400)),
                    // Block::new(Point::new(1700, 0, 0), Point::new(1800, 400, 400)),
                    // Block::new(Point::new(1800, 0, 0), Point::new(1900, 400, 400)),
                    // Block::new(Point::new(1900, 0, 0), Point::new(2000, 400, 400)),
                ];

    let mut elements: Vec<Element> = Vec::new();
    for b in blocks.iter() {
        for el in &b.elements {
            elements.push(el.clone());
        }
    }

    write_file(elements);

    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}
