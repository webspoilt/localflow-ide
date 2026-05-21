import { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useWorkspaceStore } from '@local-flow/state';
import {
  Brain,
  Shield,
  Activity,
  DollarSign,
  Play,
  RefreshCw,
  Search,
  CheckCircle,
  FolderSync,
  FileCode,
  Clock,
  Terminal,
  Info
} from 'lucide-react';

interface DagNode {
  id: string;
  name: string;
  node_type: string;
  status: string | { Failed: string };
  trigger_reason: string;
  triggered_by: string;
  input_hash: string;
  output_hash?: string;
}

interface DagEdge {
  from: string;
  to: string;
}

interface Incident {
  timestamp: string;
  severity: string;
  event_type: string;
  message: string;
  details: string;
}

interface Decision {
  id: string;
  timestamp: string;
  decision_type: string;
  choice_made: string;
  alternatives_rejected: string[];
  rationale: string;
}

interface SpeculativeAssessment {
  build_impact_risk: number;
  dependency_bloat_risk: number;
  memory_impact_mb: number;
  security_risk_score: number;
  estimated_time_ms: number;
  confidence_score: number;
}

interface StrategyPrediction {
  option_name: string;
  build_failures: number;
  test_coverage: number;
  assessment: SpeculativeAssessment;
}

interface SpeculativePrediction {
  predictions: StrategyPrediction[];
  best: {
    name: string;
    description: string;
    complexity: number;
    estimated_hours: number;
    risk: number;
  } | null;
}

interface QueryResult {
  target_id: string;
  impacted_files: string[];
  dependencies: string[];
}

interface HealthMetrics {
  architecture_quality: number;
  technical_debt_score: number;
  documentation_quality: number;
}

interface RepoHealth {
  health_score: number;
  scanned_files_count: number;
  metrics: HealthMetrics;
  aggregate_score: number;
  architecture_quality: { score: number; details: string };
  technical_debt: { score: number; details: string };
  test_coverage: { score: number; details: string };
  performance_health: { score: number; details: string };
  security_posture: { score: number; details: string };
  documentation_quality: { score: number; details: string };
  build_reliability: { score: number; details: string };
}

interface TimelinePrediction {
  upgrade_risk: number;
  predicted_bottleneck: string;
  migration_suggestion: string;
}

interface CostSummary {
  total_cost_usd: number;
  input_tokens: number;
  output_tokens: number;
  energy_wh: number;
  cpu_utilization: number;
  gpu_utilization: number;
  ram_mb: number;
}

export function CognitivePanel() {
  const [activeTab, setActiveTab] = useState<'topology' | 'cognition' | 'sandbox' | 'cost'>('topology');
  const workspaceRoot = useWorkspaceStore((s) => s.root);
  const basePath = workspaceRoot?.path ?? 'e:\\IDEAS\\agentic ide\\zynta-studio';

  // State: Topology / DAG
  const [nodes, setNodes] = useState<DagNode[]>([]);
  const [edges, setEdges] = useState<DagEdge[]>([]);
  const [dagHistory, setDagHistory] = useState<string[]>([]);
  const [simulationParams, setSimulationParams] = useState({ complexity: 2, risk: 1, hours: 4 });
  const [speculativePrediction, setSpeculativePrediction] = useState<SpeculativePrediction | null>(null);

  // State: Cognition / Architecture Graph
  const [queryFile, setQueryFile] = useState('src-tauri/src/main.rs');
  const [queryResult, setQueryResult] = useState<QueryResult | null>(null);
  const [repoHealth, setRepoHealth] = useState<RepoHealth | null>(null);
  const [timelinePrediction, setTimelinePrediction] = useState<TimelinePrediction | null>(null);

  // State: Sandbox / Security
  const [securityIncidents, setSecurityIncidents] = useState<Incident[]>([]);
  const [virtualPath, setVirtualPath] = useState('src/test_virtual.rs');
  const [virtualContent, setVirtualContent] = useState('// Virtual file test');
  const [sandboxLog, setSandboxLog] = useState<string[]>([]);

  // State: Cost & Explainability
  const [costSummary, setCostSummary] = useState<CostSummary | null>(null);
  const [decisions, setDecisions] = useState<Decision[]>([]);
  const [newDecision, setNewDecision] = useState({ type: 'Refactor', choice: 'Rust modular system', rejected: 'Single file monolithic', rationale: 'Maintain bounds' });

  // Refresh topology and DAG
  const refreshDAG = useCallback(() => {
    invoke<{ nodes: DagNode[]; edges: DagEdge[]; history: string[] }>('get_execution_graph')
      .then((res) => {
        setNodes(res.nodes);
        setEdges(res.edges);
        setDagHistory(res.history);
      })
      .catch((err: unknown) => { console.error('Failed to get DAG:', err); });
  }, []);

  // Fetch telemetry / costs
  const refreshCosts = useCallback(() => {
    invoke<CostSummary>('get_cost_summary')
      .then((res) => { setCostSummary(res); })
      .catch((err: unknown) => { console.error('Failed to get cost summary:', err); });
  }, []);

  // Fetch security logs
  const refreshSecurity = useCallback(() => {
    invoke<Incident[]>('get_security_incidents')
      .then((res) => { setSecurityIncidents(res); })
      .catch((err: unknown) => { console.error('Failed to get incidents:', err); });
  }, []);

  // Fetch decisions
  const refreshDecisions = useCallback(() => {
    invoke<Decision[]>('get_explainability_decisions')
      .then((res) => { setDecisions(res); })
      .catch((err: unknown) => { console.error('Failed to get decisions:', err); });
  }, []);

  // Fetch all initial data
  useEffect(() => {
    refreshDAG();
    refreshCosts();
    refreshSecurity();
    refreshDecisions();

    // Trigger initial health assessment
    invoke<RepoHealth>('get_repository_health', { basePath })
      .then((res) => { setRepoHealth(res); })
      .catch((err: unknown) => { console.error('Failed to get health:', err); });

    invoke<TimelinePrediction>('get_timeline_predictions')
      .then((res) => { setTimelinePrediction(res); })
      .catch((err: unknown) => { console.error('Failed to get timeline prediction:', err); });
  }, [basePath, refreshDAG, refreshCosts, refreshSecurity, refreshDecisions]);

  // Simulate executing/adding node
  const handleSimulateNode = async () => {
    const nodeTypes = ['TaskNode', 'CodeNode', 'BuildNode', 'AgentNode', 'VerificationNode', 'DecisionNode'];
    const randomType = nodeTypes[Math.floor(Math.random() * nodeTypes.length)];
    const nodeName = `Simulated ${randomType} #${(nodes.length + 1).toString()}`;
    
    try {
      await invoke('add_simulated_node', {
        name: nodeName,
        nodeTypeStr: randomType,
        triggeredBy: 'Agent System',
        reason: 'Incremental compilation check',
        inputHash: Math.random().toString(36).substring(7)
      });
      refreshDAG();
    } catch (err: unknown) {
      alert('Simulation failed: ' + String(err));
    }
  };

  // Run Speculative Simulation
  const handleSpeculativeSim = async () => {
    try {
      const res = await invoke<SpeculativePrediction>('get_speculative_predictions', {
        complexity: Number(simulationParams.complexity),
        risk: Number(simulationParams.risk),
        estimatedHours: Number(simulationParams.hours)
      });
      setSpeculativePrediction(res);
    } catch (err: unknown) {
      alert('Speculative prediction failed: ' + String(err));
    }
  };

  // Run Architecture Query
  const handleArchQuery = async () => {
    try {
      const res = await invoke<QueryResult>('query_architecture_graph', { targetId: queryFile });
      setQueryResult(res);
    } catch (err: unknown) {
      alert('Architecture query failed: ' + String(err));
    }
  };

  // Scan repository
  const handleScanRepository = async () => {
    try {
      setSandboxLog(prev => [...prev, `Scanning repository at ${basePath}...`]);
      await invoke('scan_architecture_graph', { basePath });
      setSandboxLog(prev => [...prev, 'Scan completed. Health scores recalculated.']);
      const health = await invoke<RepoHealth>('get_repository_health', { basePath });
      setRepoHealth(health);
    } catch (err: unknown) {
      setSandboxLog(prev => [...prev, `Scan failed: ${String(err)}`]);
    }
  };

  // Read virtual file
  const handleVirtualRead = async () => {
    try {
      const content = await invoke<string>('virtual_read_file', { path: virtualPath });
      setVirtualContent(content);
      setSandboxLog(prev => [...prev, `Read file virtual sandbox: ${virtualPath}`]);
    } catch (err: unknown) {
      setSandboxLog(prev => [...prev, `Virtual read error: ${String(err)}`]);
    }
  };

  // Write virtual file
  const handleVirtualWrite = async () => {
    try {
      await invoke('virtual_write_file', { path: virtualPath, content: virtualContent });
      setSandboxLog(prev => [...prev, `Wrote file virtual sandbox: ${virtualPath}`]);
      refreshSecurity(); // Leak detection might trigger new incidents
    } catch (err: unknown) {
      setSandboxLog(prev => [...prev, `Virtual write error: ${String(err)}`]);
    }
  };

  // Commit virtual changes
  const handleCommitVirtual = async () => {
    try {
      await invoke('commit_virtual_changes');
      setSandboxLog(prev => [...prev, `Commited virtual sandbox changes to real filesystem.`]);
    } catch (err: unknown) {
      setSandboxLog(prev => [...prev, `Virtual commit error: ${String(err)}`]);
    }
  };

  // Add Decision
  const handleAddDecision = async () => {
    try {
      await invoke('add_explainability_decision', {
        decisionType: newDecision.type,
        choiceMade: newDecision.choice,
        alternativesRejected: [newDecision.rejected],
        rationale: newDecision.rationale
      });
      refreshDecisions();
      setNewDecision({ type: 'Refactor', choice: '', rejected: '', rationale: '' });
    } catch (err: unknown) {
      alert('Add decision failed: ' + String(err));
    }
  };

  return (
    <div className="cognitive-dashboard">
      <style>{`
        .cognitive-dashboard {
          display: flex;
          flex-direction: column;
          height: 100%;
          background: var(--bg-primary);
          color: var(--text-primary);
          font-family: var(--font-sans);
          overflow: hidden;
        }
        .cog-tabs {
          display: flex;
          border-bottom: 1px solid var(--border-color);
          background: var(--bg-secondary);
          padding: 0 4px;
        }
        .cog-tab-btn {
          background: transparent;
          border: none;
          color: var(--text-secondary);
          padding: 8px 12px;
          cursor: pointer;
          font-size: var(--text-sm);
          display: flex;
          align-items: center;
          gap: 6px;
          border-bottom: 2px solid transparent;
          transition: all var(--transition-fast);
        }
        .cog-tab-btn:hover {
          color: var(--text-primary);
          background: var(--bg-hover);
        }
        .cog-tab-btn.active {
          color: var(--accent-blue);
          border-bottom-color: var(--accent-blue);
          background: var(--bg-active);
        }
        .cog-content {
          flex: 1;
          padding: 12px;
          overflow-y: auto;
          display: flex;
          flex-direction: column;
          gap: 16px;
        }
        .section-card {
          background: var(--bg-secondary);
          border: 1px solid var(--border-color);
          border-radius: var(--radius-md);
          padding: 12px;
          display: flex;
          flex-direction: column;
          gap: 10px;
        }
        .card-title {
          font-size: var(--text-base);
          font-weight: 600;
          display: flex;
          align-items: center;
          gap: 6px;
          border-bottom: 1px solid var(--border-subtle);
          padding-bottom: 6px;
          color: var(--text-primary);
        }
        .card-row {
          display: flex;
          justify-content: space-between;
          font-size: var(--text-sm);
        }
        .card-label {
          color: var(--text-secondary);
        }
        .card-value {
          font-weight: 500;
        }
        .status-badge {
          padding: 2px 6px;
          border-radius: var(--radius-xs);
          font-size: var(--text-xs);
          font-weight: 600;
          text-transform: uppercase;
        }
        .status-badge.completed {
          background: var(--accent-green-dim);
          color: var(--accent-green);
        }
        .status-badge.failed {
          background: var(--accent-red-dim);
          color: var(--accent-red);
        }
        .status-badge.pending {
          background: var(--bg-active);
          color: var(--text-secondary);
        }
        .cog-btn {
          background: var(--accent-blue);
          color: white;
          border: none;
          border-radius: var(--radius-sm);
          padding: 6px 12px;
          font-size: var(--text-sm);
          font-weight: 500;
          cursor: pointer;
          display: inline-flex;
          align-items: center;
          justify-content: center;
          gap: 6px;
          transition: background var(--transition-fast);
        }
        .cog-btn:hover {
          background: #3b6bd6;
        }
        .cog-btn-secondary {
          background: var(--bg-elevated);
          border: 1px solid var(--border-color);
          color: var(--text-primary);
          border-radius: var(--radius-sm);
          padding: 6px 12px;
          font-size: var(--text-sm);
          cursor: pointer;
          transition: background var(--transition-fast);
        }
        .cog-btn-secondary:hover {
          background: var(--bg-hover);
        }
        .input-group {
          display: flex;
          flex-direction: column;
          gap: 4px;
        }
        .input-group label {
          font-size: var(--text-xs);
          color: var(--text-secondary);
        }
        .cog-input {
          background: var(--bg-elevated);
          border: 1px solid var(--border-color);
          color: var(--text-primary);
          padding: 6px 8px;
          border-radius: var(--radius-sm);
          font-size: var(--text-sm);
          outline: none;
        }
        .cog-input:focus {
          border-color: var(--accent-blue);
        }
        .dag-node-item {
          display: flex;
          align-items: center;
          justify-content: space-between;
          padding: 6px 8px;
          background: var(--bg-elevated);
          border-left: 3px solid var(--border-color);
          font-size: var(--text-xs);
          border-radius: var(--radius-xs);
        }
        .dag-node-item.TaskNode { border-left-color: var(--accent-blue); }
        .dag-node-item.CodeNode { border-left-color: var(--accent-green); }
        .dag-node-item.BuildNode { border-left-color: var(--accent-yellow); }
        .dag-node-item.AgentNode { border-left-color: var(--accent-purple); }
        .dag-node-item.VerificationNode { border-left-color: #ec4899; }
        .dag-node-item.DecisionNode { border-left-color: #38bdf8; }
        .log-box {
          background: #09090d;
          border: 1px solid var(--border-color);
          border-radius: var(--radius-sm);
          padding: 8px;
          font-family: var(--font-mono);
          font-size: var(--text-xs);
          max-height: 120px;
          overflow-y: auto;
          color: var(--text-secondary);
        }
        .health-bar {
          height: 8px;
          background: var(--bg-elevated);
          border-radius: 4px;
          overflow: hidden;
        }
        .health-fill {
          height: 100%;
          border-radius: 4px;
          transition: width 0.3s ease;
        }
      `}</style>

      {/* Tabs list */}
      <div className="cog-tabs">
        <button
          className={`cog-tab-btn ${activeTab === 'topology' ? 'active' : ''}`}
          onClick={() => { setActiveTab('topology'); }}
        >
          <Activity size={14} /> Topology & Spec
        </button>
        <button
          className={`cog-tab-btn ${activeTab === 'cognition' ? 'active' : ''}`}
          onClick={() => { setActiveTab('cognition'); }}
        >
          <Brain size={14} /> Cognition & Health
        </button>
        <button
          className={`cog-tab-btn ${activeTab === 'sandbox' ? 'active' : ''}`}
          onClick={() => { setActiveTab('sandbox'); }}
        >
          <Shield size={14} /> Sandbox & Sec
        </button>
        <button
          className={`cog-tab-btn ${activeTab === 'cost' ? 'active' : ''}`}
          onClick={() => { setActiveTab('cost'); }}
        >
          <DollarSign size={14} /> Governance & Exp
        </button>
      </div>

      <div className="cog-content">
        {activeTab === 'topology' && (
          <>
            {/* Execution Graph DAG Nodes */}
            <div className="section-card">
              <div className="card-title">
                <Play size={14} className="text-accent-blue" />
                Execution Graph (DAG) ({nodes.length.toString()} nodes, {edges.length.toString()} edges)
              </div>
              <div style={{ display: 'flex', gap: '8px', marginBottom: '8px' }}>
                <button className="cog-btn" onClick={() => { void handleSimulateNode(); }}>
                  <Play size={12} /> Spawn Simulated Node
                </button>
                <button className="cog-btn-secondary" onClick={() => { refreshDAG(); }}>
                  <RefreshCw size={12} /> Sync
                </button>
              </div>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', maxHeight: '180px', overflowY: 'auto' }}>
                {nodes.length === 0 ? (
                  <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)' }}>
                    No execution nodes yet. Click the button above to simulate.
                  </div>
                ) : (
                  nodes.map((node) => {
                    const nodeTypeClass = node.node_type;
                    const statusStr = typeof node.status === 'string' ? node.status : 'Failed';
                    return (
                      <div key={node.id} className={`dag-node-item ${nodeTypeClass}`}>
                        <div>
                          <div style={{ fontWeight: 600 }}>{node.name}</div>
                          <div style={{ color: 'var(--text-muted)', fontSize: '10px' }}>
                            Triggered by {node.triggered_by} ({node.trigger_reason})
                          </div>
                        </div>
                        <span className={`status-badge ${statusStr.toLowerCase()}`}>
                          {statusStr}
                        </span>
                      </div>
                    );
                  })
                )}
              </div>
            </div>

            {/* Speculative Simulation Engine */}
            <div className="section-card">
              <div className="card-title">
                <Clock size={14} />
                Speculative Simulator (Plan Risks)
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '8px' }}>
                <div className="input-group">
                  <label>Complexity (1-5)</label>
                  <input
                    type="number"
                    min="1"
                    max="5"
                    className="cog-input"
                    value={simulationParams.complexity}
                    onChange={(e) => { setSimulationParams({ ...simulationParams, complexity: Number(e.target.value) }); }}
                  />
                </div>
                <div className="input-group">
                  <label>Risk (1-5)</label>
                  <input
                    type="number"
                    min="1"
                    max="5"
                    className="cog-input"
                    value={simulationParams.risk}
                    onChange={(e) => { setSimulationParams({ ...simulationParams, risk: Number(e.target.value) }); }}
                  />
                </div>
                <div className="input-group">
                  <label>Est Hours</label>
                  <input
                    type="number"
                    min="1"
                    max="20"
                    className="cog-input"
                    value={simulationParams.hours}
                    onChange={(e) => { setSimulationParams({ ...simulationParams, hours: Number(e.target.value) }); }}
                  />
                </div>
              </div>
              <button className="cog-btn" onClick={() => { void handleSpeculativeSim(); }}>
                Run Risk Simulation
              </button>

              {speculativePrediction && (
                <div style={{ background: 'var(--bg-elevated)', padding: '8px', borderRadius: '4px', fontSize: 'var(--text-xs)', display: 'flex', flexDirection: 'column', gap: '4px' }}>
                  <div style={{ fontWeight: 600, color: 'var(--accent-blue)', marginBottom: '2px' }}>
                    Predicted Impact for "{speculativePrediction.best?.name ?? 'Balanced'}" Approach:
                  </div>
                  {speculativePrediction.predictions.map((pred: StrategyPrediction, idx: number) => (
                    <div key={idx} style={{ borderBottom: '1px solid var(--border-subtle)', paddingBottom: '4px', marginBottom: '4px' }}>
                      <div style={{ fontWeight: 500 }}>{pred.option_name} Path</div>
                      <div className="card-row">
                        <span className="card-label">Build Impact Risk:</span>
                        <span className="card-value">{Math.round(pred.assessment.build_impact_risk * 100)}%</span>
                      </div>
                      <div className="card-row">
                        <span className="card-label">Dependency Bloat:</span>
                        <span className="card-value">{Math.round(pred.assessment.dependency_bloat_risk * 100)}%</span>
                      </div>
                      <div className="card-row">
                        <span className="card-label">VRAM Context Use:</span>
                        <span className="card-value">{pred.assessment.memory_impact_mb} MB</span>
                      </div>
                      <div className="card-row">
                        <span className="card-label">Security Risk Score:</span>
                        <span className="card-value">{Math.round(pred.assessment.security_risk_score * 100)}%</span>
                      </div>
                      <div className="card-row" style={{ fontWeight: 'bold' }}>
                        <span className="card-label" style={{ color: 'var(--text-primary)' }}>Execution Confidence:</span>
                        <span className="card-value" style={{ color: pred.assessment.confidence_score > 0.7 ? 'var(--accent-green)' : 'var(--accent-yellow)' }}>
                          {Math.round(pred.assessment.confidence_score * 100)}%
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {/* DAG History Logs */}
            <div className="section-card">
              <div className="card-title">
                <Terminal size={14} /> Lineage Logs
              </div>
              <div className="log-box">
                {dagHistory.map((log, index) => (
                  <div key={index}>{log}</div>
                ))}
              </div>
            </div>
          </>
        )}

        {activeTab === 'cognition' && (
          <>
            {/* Architecture Scan and Queries */}
            <div className="section-card">
              <div className="card-title">
                <FolderSync size={14} />
                Knowledge Graph Engine
              </div>
              <div style={{ display: 'flex', gap: '8px' }}>
                <button className="cog-btn" onClick={() => { void handleScanRepository(); }} style={{ flex: 1 }}>
                  Scan Repository codebase
                </button>
              </div>

              <div className="input-group" style={{ marginTop: '8px' }}>
                <label>Query Affected Dependencies</label>
                <div style={{ display: 'flex', gap: '4px' }}>
                  <input
                    type="text"
                    className="cog-input"
                    style={{ flex: 1 }}
                    value={queryFile}
                    onChange={(e) => { setQueryFile(e.target.value); }}
                  />
                  <button className="cog-btn-secondary" onClick={() => { void handleArchQuery(); }}>
                    <Search size={14} />
                  </button>
                </div>
              </div>

              {queryResult && (
                <div style={{ background: 'var(--bg-elevated)', padding: '8px', borderRadius: '4px', fontSize: 'var(--text-xs)' }}>
                  <div style={{ fontWeight: 600, color: 'var(--accent-blue)', marginBottom: '4px' }}>
                    Impacted Files if {queryResult.target_id} changes:
                  </div>
                  {queryResult.impacted_files.length === 0 ? (
                    <div style={{ color: 'var(--text-muted)' }}>None detected (independent module)</div>
                  ) : (
                    <ul style={{ paddingLeft: '16px', margin: '4px 0' }}>
                      {queryResult.impacted_files.map((f: string, i: number) => (
                        <li key={i}>{f}</li>
                      ))}
                    </ul>
                  )}
                  <div style={{ fontWeight: 600, color: 'var(--accent-green)', marginTop: '6px', marginBottom: '4px' }}>
                    Dependencies of {queryResult.target_id}:
                  </div>
                  {queryResult.dependencies.length === 0 ? (
                    <div style={{ color: 'var(--text-muted)' }}>None detected</div>
                  ) : (
                    <ul style={{ paddingLeft: '16px', margin: '4px 0' }}>
                      {queryResult.dependencies.map((f: string, i: number) => (
                        <li key={i}>{f}</li>
                      ))}
                    </ul>
                  )}
                </div>
              )}
            </div>

            {/* Codebase Health Score */}
            <div className="section-card">
              <div className="card-title">
                <Activity size={14} />
                Repository Health Engine
              </div>
              {repoHealth ? (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  <div>
                    <div className="card-row" style={{ marginBottom: '4px' }}>
                      <span className="card-label">Overall Health Score:</span>
                      <span className="card-value" style={{ color: repoHealth.health_score > 75 ? 'var(--accent-green)' : 'var(--accent-yellow)' }}>
                        {repoHealth.health_score.toString()}/100
                      </span>
                    </div>
                    <div className="health-bar">
                      <div
                        className="health-fill"
                        style={{
                          width: `${repoHealth.health_score.toString()}%`,
                          backgroundColor: repoHealth.health_score > 75 ? 'var(--accent-green)' : 'var(--accent-yellow)'
                        }}
                      />
                    </div>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Files Scanned:</span>
                    <span className="card-value">{repoHealth.scanned_files_count}</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Architecture Quality:</span>
                    <span className="card-value">{repoHealth.metrics.architecture_quality}/100</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Estimated Technical Debt:</span>
                    <span className="card-value">{repoHealth.metrics.technical_debt_score} / 100</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Documentation Ratio:</span>
                    <span className="card-value">{repoHealth.metrics.documentation_quality}/100</span>
                  </div>
                </div>
              ) : (
                <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)' }}>
                  Loading health metrics... Scan repository above to refresh.
                </div>
              )}
            </div>

            {/* Temporal Timeline Evolution */}
            <div className="section-card">
              <div className="card-title">
                <Clock size={14} />
                Timeline Forecasting (Maintenance)
              </div>
              {timelinePrediction ? (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', fontSize: 'var(--text-xs)' }}>
                  <div className="card-row">
                    <span className="card-label">Upgrade Risk score:</span>
                    <span className="card-value" style={{ color: 'var(--accent-yellow)' }}>
                      {Math.round(timelinePrediction.upgrade_risk * 100)}%
                    </span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Bottleneck Forecast:</span>
                    <span className="card-value" style={{ textTransform: 'capitalize' }}>
                      {timelinePrediction.predicted_bottleneck.replace('_', ' ')}
                    </span>
                  </div>
                  <div style={{ background: 'var(--bg-elevated)', padding: '6px', borderRadius: '4px', marginTop: '4px', borderLeft: '2px solid var(--accent-yellow)' }}>
                    <div style={{ fontWeight: 600 }}>Upgrade Suggestion:</div>
                    <div style={{ color: 'var(--text-secondary)' }}>{timelinePrediction.migration_suggestion}</div>
                  </div>
                </div>
              ) : (
                <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)' }}>
                  Calculating forecasts...
                </div>
              )}
            </div>
          </>
        )}

        {activeTab === 'sandbox' && (
          <>
            {/* Virtual File Sandbox edits */}
            <div className="section-card">
              <div className="card-title">
                <FileCode size={14} />
                Virtual File Sandbox V2
              </div>
              <div className="input-group">
                <label>Virtual File Path</label>
                <input
                  type="text"
                  className="cog-input"
                  value={virtualPath}
                  onChange={(e) => { setVirtualPath(e.target.value); }}
                />
              </div>
              <div className="input-group">
                <label>Content (edit securely)</label>
                <textarea
                  className="cog-input"
                  style={{ fontFamily: 'var(--font-mono)', minHeight: '80px', resize: 'vertical' }}
                  value={virtualContent}
                  onChange={(e) => { setVirtualContent(e.target.value); }}
                />
              </div>
              <div style={{ display: 'flex', gap: '6px' }}>
                <button className="cog-btn" style={{ flex: 1 }} onClick={() => { void handleVirtualWrite(); }}>
                  Sandbox Write
                </button>
                <button className="cog-btn-secondary" style={{ flex: 1 }} onClick={() => { void handleVirtualRead(); }}>
                  Sandbox Read
                </button>
              </div>
              <button className="cog-btn" style={{ background: 'var(--accent-green)' }} onClick={() => { void handleCommitVirtual(); }}>
                Commit Virtual Sandbox
              </button>
            </div>

            {/* Security Alerts and Secret Detector */}
            <div className="section-card">
              <div className="card-title">
                <Shield size={14} className="text-accent-red" />
                Security Auditor & Secret Leak Detector
              </div>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                {securityIncidents.length === 0 ? (
                  <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)', display: 'flex', alignItems: 'center', gap: '4px' }}>
                    <CheckCircle size={12} className="text-accent-green" /> No filesystem sandbox or credential leak incidents.
                  </div>
                ) : (
                  securityIncidents.map((inc, i) => (
                    <div key={i} style={{ background: 'rgba(248, 113, 113, 0.08)', border: '1px solid var(--accent-red-dim)', padding: '6px', borderRadius: '4px', fontSize: 'var(--text-xs)' }}>
                      <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 600, color: 'var(--accent-red)' }}>
                        <span>{inc.event_type}</span>
                        <span style={{ fontSize: '10px', textTransform: 'uppercase' }}>{inc.severity}</span>
                      </div>
                      <div style={{ margin: '2px 0' }}>{inc.message}</div>
                      <div style={{ fontSize: '10px', color: 'var(--text-secondary)' }}>{inc.details}</div>
                    </div>
                  ))
                )}
              </div>
            </div>

            {/* Sandbox System Log */}
            <div className="section-card">
              <div className="card-title">
                <Terminal size={14} /> Virtual Environment Logs
              </div>
              <div className="log-box" style={{ maxHeight: '100px' }}>
                {sandboxLog.length === 0 ? (
                  <div>Virtual sandbox inactive. Perform writes to trigger validation check logs.</div>
                ) : (
                  sandboxLog.map((log, i) => (
                    <div key={i}>{log}</div>
                  ))
                )}
              </div>
            </div>
          </>
        )}

        {activeTab === 'cost' && (
          <>
            {/* Cost Intelligence and hardware utilization */}
            <div className="section-card">
              <div className="card-title">
                <DollarSign size={14} />
                Cost & Resource Governance
              </div>
              <button className="cog-btn-secondary" onClick={() => { refreshCosts(); }} style={{ marginBottom: '4px' }}>
                <RefreshCw size={12} /> Sync Telemetry
              </button>
              {costSummary ? (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                  <div className="card-row">
                    <span className="card-label">Cumulative Cost (USD):</span>
                    <span className="card-value" style={{ color: 'var(--accent-green)', fontWeight: 'bold' }}>
                      ${costSummary.total_cost_usd.toFixed(4)}
                    </span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Input Tokens:</span>
                    <span className="card-value">{costSummary.input_tokens}</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Output Tokens:</span>
                    <span className="card-value">{costSummary.output_tokens}</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">Est Energy Consumption:</span>
                    <span className="card-value">{costSummary.energy_wh.toFixed(4)} Wh</span>
                  </div>
                  <div style={{ borderTop: '1px solid var(--border-subtle)', paddingTop: '6px', marginTop: '4px' }} />
                  <div className="card-row">
                    <span className="card-label">CPU Utilization:</span>
                    <span className="card-value">{costSummary.cpu_utilization.toFixed(1)}%</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">GPU/VRAM Utilization:</span>
                    <span className="card-value">{costSummary.gpu_utilization.toFixed(1)}%</span>
                  </div>
                  <div className="card-row">
                    <span className="card-label">RAM Consumption:</span>
                    <span className="card-value">{costSummary.ram_mb} MB</span>
                  </div>
                </div>
              ) : (
                <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)' }}>
                  Loading cost telemetry...
                </div>
              )}
            </div>

            {/* Explainability Engine */}
            <div className="section-card">
              <div className="card-title">
                <Info size={14} />
                Cognitive Explainability Rationale
              </div>

              {/* Rationale logger form */}
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', background: 'var(--bg-elevated)', padding: '8px', borderRadius: '4px', fontSize: 'var(--text-xs)' }}>
                <div style={{ fontWeight: 600, marginBottom: '4px' }}>Log Custom AI Decision Trace</div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '4px' }}>
                  <input
                    type="text"
                    placeholder="Type (e.g. Model)"
                    className="cog-input"
                    value={newDecision.type}
                    onChange={(e) => { setNewDecision({ ...newDecision, type: e.target.value }); }}
                  />
                  <input
                    type="text"
                    placeholder="Choice Made"
                    className="cog-input"
                    value={newDecision.choice}
                    onChange={(e) => { setNewDecision({ ...newDecision, choice: e.target.value }); }}
                  />
                </div>
                <input
                  type="text"
                  placeholder="Alternative Rejected"
                  className="cog-input"
                  value={newDecision.rejected}
                  onChange={(e) => { setNewDecision({ ...newDecision, rejected: e.target.value }); }}
                />
                <textarea
                  placeholder="Rationale justification"
                  className="cog-input"
                  style={{ minHeight: '40px', resize: 'vertical' }}
                  value={newDecision.rationale}
                  onChange={(e) => { setNewDecision({ ...newDecision, rationale: e.target.value }); }}
                />
                <button className="cog-btn" onClick={() => { void handleAddDecision(); }}>
                  Add Rationale Entry
                </button>
              </div>

              {/* Recorded Decisions list */}
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', marginTop: '6px' }}>
                {decisions.length === 0 ? (
                  <div style={{ fontSize: 'var(--text-xs)', color: 'var(--text-muted)' }}>
                    No decisions recorded yet. Add one above to create a cognitive trace.
                  </div>
                ) : (
                  decisions.map((dec) => (
                    <div key={dec.id} style={{ background: 'var(--bg-elevated)', padding: '8px', borderRadius: '4px', fontSize: 'var(--text-xs)' }}>
                      <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 600 }}>
                        <span style={{ color: 'var(--accent-blue)' }}>{dec.decision_type} Decision</span>
                        <span style={{ color: 'var(--text-muted)', fontSize: '10px' }}>{new Date(dec.timestamp).toLocaleTimeString()}</span>
                      </div>
                      <div style={{ marginTop: '2px' }}>
                        <strong>Selected:</strong> {dec.choice_made}
                      </div>
                      <div>
                        <strong>Rejected:</strong> {dec.alternatives_rejected.join(', ')}
                      </div>
                      <div style={{ marginTop: '4px', paddingLeft: '4px', borderLeft: '2px solid var(--border-color)', color: 'var(--text-secondary)' }}>
                        "{dec.rationale}"
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
