use localflow_ide::scheduler::{TaskQueue, TaskDefinition, TaskPriority};
use tokio::sync::mpsc;
use localflow_ide::events::RuntimeEvent;

#[tokio::test]
async fn test_enqueue_dequeue() {
    let (event_tx, _event_rx) = mpsc::unbounded_channel::<RuntimeEvent>();
    let queue = TaskQueue::new(event_tx);

    let id = queue.enqueue(TaskDefinition {
        id: uuid::Uuid::new_v4(),
        command: Some("echo hello".into()),
        args: vec![],
        cwd: None,
        env: std::collections::HashMap::new(),
        timeout_ms: 5000,
        max_retries: 2,
        priority: TaskPriority::Normal,
    });

    let task = queue.dequeue().await;
    assert!(task.is_some());
    assert_eq!(task.unwrap().id, id);
}

#[tokio::test]
async fn test_queue_len() {
    let (event_tx, _event_rx) = mpsc::unbounded_channel::<RuntimeEvent>();
    let queue = TaskQueue::new(event_tx);

    assert_eq!(queue.len(), 0);

    let def = TaskDefinition {
        id: uuid::Uuid::new_v4(),
        command: Some("test".into()),
        args: vec![],
        cwd: None,
        env: std::collections::HashMap::new(),
        timeout_ms: 1000,
        max_retries: 0,
        priority: TaskPriority::Normal,
    };

    queue.enqueue(def.clone());
    assert_eq!(queue.len(), 1);

    queue.dequeue().await;
    assert_eq!(queue.len(), 0);
}
