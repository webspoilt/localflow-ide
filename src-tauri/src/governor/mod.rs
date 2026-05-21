use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct ResourceGovernor {
    max_memory_mb: u64,
    max_cpu_percent: u8,
    max_parallel_jobs: usize,
    active_jobs: Arc<Mutex<usize>>,
    used_memory_mb: Arc<Mutex<u64>>,
}

#[derive(Clone, Debug)]
pub struct ResourceUsage {
    pub memory_mb: u64,
    pub active_jobs: usize,
    pub cpu_percent: u8,
}

impl ResourceGovernor {
    pub fn new(max_memory_mb: u64, max_cpu_percent: u8, max_parallel_jobs: usize) -> Self {
        info!(
            "ResourceGovernor: {}MB max, {}% CPU, {} parallel jobs",
            max_memory_mb, max_cpu_percent, max_parallel_jobs
        );
        Self {
            max_memory_mb,
            max_cpu_percent,
            max_parallel_jobs,
            active_jobs: Arc::new(Mutex::new(0)),
            used_memory_mb: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn try_acquire(&self, estimated_memory_mb: u64) -> Result<ResourceLease, String> {
        let mut jobs = self.active_jobs.lock().await;
        let mut mem = self.used_memory_mb.lock().await;

        if *jobs >= self.max_parallel_jobs {
            return Err("Max parallel jobs reached".into());
        }
        if *mem + estimated_memory_mb > self.max_memory_mb {
            return Err("Memory limit would be exceeded".into());
        }

        *jobs += 1;
        *mem += estimated_memory_mb;

        Ok(ResourceLease {
            governor: self,
            memory_mb: estimated_memory_mb,
        })
    }

    pub async fn usage(&self) -> ResourceUsage {
        ResourceUsage {
            memory_mb: *self.used_memory_mb.lock().await,
            active_jobs: *self.active_jobs.lock().await,
            cpu_percent: self.max_cpu_percent,
        }
    }

    async fn release(&self, memory_mb: u64) {
        let mut jobs = self.active_jobs.lock().await;
        let mut mem = self.used_memory_mb.lock().await;
        *jobs = jobs.saturating_sub(1);
        *mem = mem.saturating_sub(memory_mb);
    }
}

pub struct ResourceLease {
    governor: *const ResourceGovernor,
    memory_mb: u64,
}

impl Drop for ResourceLease {
    fn drop(&mut self) {
        let governor = unsafe { &*self.governor };
        let memory_mb = self.memory_mb;
        tokio::spawn(async move {
            governor.release(memory_mb).await;
        });
    }
}

unsafe impl Send for ResourceLease {}
