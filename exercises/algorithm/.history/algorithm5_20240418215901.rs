/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

struct Queue{
    items: Vec<usize>,
    front: usize,
    rear: usize,
}

impl Queue{
    fn new() -> Queue{
        Queue{
            items:Vec::new(),
            front: 0,
            rear: 0,
        }
    }

    fn enqueue(&mut self,value: usize){
        self.items.push(value);
        self.rear+=1;
    }

    fn dequeue(&mut self) -> Option<usize>{
        if self.front < self.rear{
            let value = self.items[self.front];
            self.front+=1;
            Some(value)
        }
        else{
            None
        }
    }

    fn is_empty(&self) -> bool{
        self.front == self.rear
    }
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }
/*
    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
            let mut visited = vec![false; self.adj.len()];
            let mut queue = VecDeque::new();
            let mut order = vec![];
    
            visited[start] = true;
            queue.push_back(start);
    
            while let Some(node) = queue.pop_front() {
                order.push(node);
    
                for &neighbour in &self.adj[node] {
                    if !visited[neighbour] {
                        visited[neighbour] = true;
                        queue.push_back(neighbour);
                    }
                }
            }
    
            order
        }
        */
        fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
            let mut q = Queue::new();
            let mut visited = vec![false;self.adj.len()];
            let mut visit_order = Vec::new();
    
            q.enqueue(start);
            visited[start] = true;
    
            while !Queue::is_empty(&q) {
                if let Some(node) = q.dequeue(){
                    visit_order.push(node);
    
                    for &neighbor in &self.adj[node] {
                        if !visited[neighbor] {
                            q.enqueue(neighbor);
                            visited[neighbor] = true;
                        }
                    }
                }
            }
            visit_order
        }
    }
    
    
    



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

