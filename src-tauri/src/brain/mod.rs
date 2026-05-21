pub mod context;
pub mod goal;
pub mod questions;
pub mod options;
pub mod matrix;
pub mod simulator;
pub mod dag;
pub mod dispatch;

use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

use crate::events::RuntimeEvent;
use crate::scheduler::{TaskDefinition, TaskPriority};

pub struct Brain {
    context: context::ContextCollector,
    goal_parser: goal::GoalParser,
    question_gen: questions::QuestionGenerator,
    option_gen: options::OptionGenerator,
    matrix: matrix::DecisionMatrix,
    simulator: simulator::StrategySimulator,
    dag_gen: dag::TaskDagGenerator,
    dispatcher: dispatch::AgentDispatcher,
    event_sender: mpsc::UnboundedSender<RuntimeEvent>,
}

impl Brain {
    pub fn new(event_sender: mpsc::UnboundedSender<RuntimeEvent>) -> Self {
        info!("Cognitive Brain initialized");
        Self {
            context: context::ContextCollector::new(),
            goal_parser: goal::GoalParser::new(),
            question_gen: questions::QuestionGenerator::new(),
            option_gen: options::OptionGenerator::new(),
            matrix: matrix::DecisionMatrix::new(),
            simulator: simulator::StrategySimulator::new(),
            dag_gen: dag::TaskDagGenerator::new(),
            dispatcher: dispatch::AgentDispatcher::new(event_sender.clone()),
            event_sender,
        }
    }

    pub async fn process_goal(&self, raw_goal: &str) -> BrainResult {
        info!(goal = %raw_goal, "Brain processing goal");

        let parsed = self.goal_parser.parse(raw_goal);
        let ctx = self.context.collect(raw_goal).await;
        let questions = self.question_gen.generate(&parsed, &ctx);
        let options = self.option_gen.generate(&parsed, &ctx);
        let mut matrix = self.matrix.clone();
        let scores = matrix.evaluate(&options);
        let simulated = self.simulator.simulate(&options, &scores);
        let dag = self.dag_gen.generate(&simulated.best);

        let tasks = self.dispatcher.dispatch(&dag).await;

        BrainResult {
            goal_id: Uuid::new_v4(),
            questions,
            options,
            scores,
            simulation: simulated,
            dag,
            tasks,
        }
    }

    pub async fn plan(&self, goal: &str) -> Vec<TaskDefinition> {
        let result = self.process_goal(goal).await;
        result.tasks
    }
}

pub struct BrainResult {
    pub goal_id: Uuid,
    pub questions: Vec<String>,
    pub options: Vec<options::OptionStrategy>,
    pub scores: Vec<matrix::ScoredOption>,
    pub simulation: simulator::SimulationResult,
    pub dag: dag::TaskDag,
    pub tasks: Vec<TaskDefinition>,
}
