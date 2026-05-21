use crate::brain::options::OptionStrategy;
use crate::scheduler::TaskPriority;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TaskDag {
    pub nodes: Vec<DagNode>,
    pub edges: Vec<DagEdge>,
}

#[derive(Clone, Debug)]
pub struct DagNode {
    pub id: Uuid,
    pub label: String,
    pub phase: u32,
}

#[derive(Clone, Debug)]
pub struct DagEdge {
    pub from: Uuid,
    pub to: Uuid,
}

pub struct TaskDagGenerator;

impl TaskDagGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, _option: &OptionStrategy) -> TaskDag {
        let analysis = DagNode { id: Uuid::new_v4(), label: "analyze".into(), phase: 0 };
        let planning = DagNode { id: Uuid::new_v4(), label: "plan".into(), phase: 1 };
        let execution = DagNode { id: Uuid::new_v4(), label: "execute".into(), phase: 2 };
        let verify = DagNode { id: Uuid::new_v4(), label: "verify".into(), phase: 3 };

        TaskDag {
            nodes: vec![analysis.clone(), planning.clone(), execution.clone(), verify.clone()],
            edges: vec![
                DagEdge { from: analysis.id, to: planning.id },
                DagEdge { from: planning.id, to: execution.id },
                DagEdge { from: execution.id, to: verify.id },
            ],
        }
    }
}

impl TaskDag {
    pub fn topo_sort(&self) -> Vec<DagNode> {
        let mut sorted = self.nodes.clone();
        sorted.sort_by_key(|n| n.phase);
        sorted
    }
}
