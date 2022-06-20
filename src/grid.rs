use std::collections::HashMap;

use crate::cell::Cell;

struct Node<T> {
    value: &T,
    edges: Vec<EdgeId>
}

struct NodeId {
    id: usize
}

struct Edge<T> {
    value: &T,
    target: NodeId
}

struct EdgeId {
    id: usize
}

struct Graph<NT, ET> {
    nodes: Vec<Node<NT>>,
    edges: Vec<Edge<ET>>
}


impl Graph<NT, ET> {
    pub fn new() -> Graph<NT, ET> {
        let mut nodes = Vec<NT>::new();
        let mut edges = Vec<ET>::new();

        {nodes, edges}
    }
}

struct Grid {
    size: u32,
    graph: Graph<Cell, Void>
}

impl Grid {
    pub fn new(size: u32) -> Grid {
        let graph = Graph::new();
        
        populate_cells(&mut graph, size);

        Grid { size, graph }
    }
}

fn populate_cells(graph: &mut Graph, size: u32) {
    for row in 0..size {
        for column in 0..size {
            let cell = Cell::new(row, column);
            graph.add_node(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid = Grid::new(5);
        assert_eq!(grid.size, 5);
        assert_eq!(grid.cells.len(), 5 * 5);
    }
}
