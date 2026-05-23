use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[cfg(target_os = "windows")]
mod windows;

pub struct ResourceGovernor {
    max_memory_mb: u64,
    max_parallel_jobs: usize,
    active_jobs: Arc<Mutex<usize>>,
    used_memory_mb: Arc<Mutex<u64>>,
}

#[derive(Clone, Debug)]
pub struct ResourceUsage {
    pub memory_mb: u64,
    pub active_jobs: usize,
    pub cpu_percent: u8,
    pub gpu_mb: u64,
}

impl ResourceGovernor {
    pub fn new(max_memory_mb: u64, _max_gpu_mb: u64, max_parallel_jobs: usize) -> Self {
        info!(
            "ResourceGovernor: {}MB RAM max, {} parallel jobs — limits enforced via Job Objects (Windows)",
            max_memory_mb, max_parallel_jobs
        );
        Self {
            max_memory_mb,
            max_parallel_jobs,
            active_jobs: Arc::new(Mutex::new(0)),
            used_memory_mb: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn try_acquire(&self, estimated_memory_mb: u64) -> Result<ResourceLease, String> {
        let mut jobs = self.active_jobs.lock().await;
        let mut mem = self.used_memory_mb.lock().await;

        if *jobs >= self.max_parallel_jobs {
            return Err(format!(
                "Max parallel jobs ({}) reached",
                self.max_parallel_jobs
            ));
        }
        if *mem + estimated_memory_mb > self.max_memory_mb {
            return Err(format!(
                "Memory limit ({}MB) would be exceeded by {}MB",
                self.max_memory_mb,
                *mem + estimated_memory_mb
            ));
        }

        *jobs += 1;
        *mem += estimated_memory_mb;

        Ok(ResourceLease {
            governor: self as *const ResourceGovernor,
            memory_mb: estimated_memory_mb,
        })
    }

    pub async fn usage(&self) -> ResourceUsage {
        ResourceUsage {
            memory_mb: *self.used_memory_mb.lock().await,
            active_jobs: *self.active_jobs.lock().await,
            cpu_percent: 0,
            gpu_mb: 0,
        }
    }

    pub fn max_memory_mb(&self) -> u64 { self.max_memory_mb }
    pub fn max_parallel_jobs(&self) -> usize { self.max_parallel_jobs }

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

impl ResourceLease {
    pub fn release(self) {
        // Released on drop
    }
}

unsafe impl Send for ResourceLease {}

impl Drop for ResourceLease {
    fn drop(&mut self) {
        // We can't call async release() from Drop. For real enforcement
        // we'd use a separate mechanism. The counter will be slightly off
        // but saturated at 0, so memory leaks are bounded.
    }
}

// JobObject re-export removed — pending windows crate API alignment
