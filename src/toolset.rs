pub enum IOSwitch {
    Input,
    Output
}

pub struct Node {
    name: String,
    io: IOSwitch,
}

pub struct Line {
    name: String,
    input_node: Node,
    output_nodes: Vec<Node>
}

pub struct Area {
    name: String,
    node_vec: Vec<Node>,
    line_vec: Vec<Space>,
}

pub struct Space {
    pub area_vec: Vec<Area>,
    pub line_vec: Vec<Area>,
}
