use std::collections::HashMap;
use std::iter;

type Arity = usize;
type Id = usize;

#[derive(Clone)]
struct Symbol {
    name: String,
    arity: Arity
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.arity)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Port {
    Free(Arity),
    Principal(Id),
    Auxiliary(Id, Arity)
}

use Port::*;

#[derive(Debug)]
struct Net {
    arity: Arity,
    cells: Vec<Symbol>,
    wires: HashMap<Port, Port>
}

impl Net {
    fn new(arity: Arity) -> Self {
        Net {
            arity: arity,
            cells: vec![],
            wires: HashMap::new(),
        }
    }

    fn add_cell(&mut self, symbol: Symbol) -> Id {
        let id = self.cells.len();
        self.cells.push(symbol);
        id
    }

    fn connect(&mut self, port_1: Port, port_2: Port) {
        self.wires.insert(port_1.clone(), port_2.clone());
        self.wires.insert(port_2.clone(), port_1.clone());
    }

    fn free_ports(&self) -> impl Iterator<Item=Port> {
        (0..self.arity).map(Free)
    }

    fn get_cell(&self, cell_id: Id) -> Option<(&Symbol, impl Iterator<Item=Port> + '_)> {
        self.cells.get(cell_id).map(|sym| {
            let principal = iter::once(Principal(cell_id));
            let auxiliary = (0..sym.arity).map(move |i| Auxiliary(cell_id, i));
            (sym, principal.chain(auxiliary))
        })
    }

    fn connected_port(&self, port: &Port) -> Option<&Port> {
        self.wires.get(port)
    }
}

fn main() {
    let plus: Symbol = Symbol{name: "+".to_string(), arity: 2};
    let mut net = Net::new(3);
    let cell_id = net.add_cell(plus);
    net.connect(Free(0), Principal(cell_id));
    net.connect(Free(1), Auxiliary(cell_id, 0));
    net.connect(Free(2), Auxiliary(cell_id, 1));
    for port in net.free_ports() {
        let connected_port = net.connected_port(&port);
        println!("{:?} -> {:?}", port, connected_port)
    }
}