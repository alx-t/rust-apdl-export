use std::collections::HashMap;

use point::Point;

#[derive(Clone)]
pub struct Element {
    pub number: usize,
    pub coord_start: Point,
    pub coord_end: Point,
    pub nodes: [Point; 8]
}

impl Element {
    pub fn new(number: usize, coord_start: Point, coord_end: Point) -> Element {
        Element {
            number: number,
            coord_start: coord_start,
            coord_end: coord_end,
            nodes: Element::nodes(coord_start, coord_end)
        }
    }

    pub fn nodes(coord_start: Point, coord_end: Point) -> [Point; 8] {
        [
            coord_end,
            Point::new(coord_end.x, coord_start.y, coord_end.z),
            Point::new(coord_start.x, coord_start.y, coord_end.z),
            Point::new(coord_start.x, coord_end.y, coord_end.z),
            Point::new(coord_end.x, coord_end.y, coord_start.z),
            Point::new(coord_end.x, coord_start.y, coord_start.z),
            coord_start,
            Point::new(coord_start.x, coord_end.y, coord_start.z)
        ]
    }

    pub fn nodes_nums(&self, node: &Node) -> [usize; 8] {
        [
            node.get_node_number(self.nodes[0]),
            node.get_node_number(self.nodes[1]),
            node.get_node_number(self.nodes[2]),
            node.get_node_number(self.nodes[3]),
            node.get_node_number(self.nodes[4]),
            node.get_node_number(self.nodes[5]),
            node.get_node_number(self.nodes[6]),
            node.get_node_number(self.nodes[7])
        ]
    }
}

pub struct SingleNode {
    pub num: usize,
    pub x: i32,
    pub y: i32,
    pub z: i32
}

pub struct Node {
    pub map: HashMap<String, SingleNode>
}

impl Node {
    pub fn new(elements: &Vec<Element>) -> Node {
        Node {
            map: Node::create_map(elements)
        }
    }

    fn create_map(elements: &Vec<Element>) -> HashMap<String, SingleNode> {
        let mut result: HashMap<String, SingleNode> = HashMap::new();

        for elem in elements.iter() {
            for nd in elem.nodes.iter() {
                let key = format!("{}-{}-{}", nd.x, nd.y, nd.z);
                let count = result.len() + 1;
                let _ = result.entry(key).or_insert(SingleNode { num: count, x: nd.x, y: nd.y, z: nd.z });
            }
        }
        result
    }

    pub fn get_node_number(&self, coord: Point) -> usize {
        match self.map.get(&format!("{}-{}-{}", coord.x, coord.y, coord.z)) {
            Some(result) => result.num,
            None => 0
        }
    }
}
