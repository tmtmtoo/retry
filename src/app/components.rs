use crate::config::*;
use crate::exec::*;
use crate::prelude::*;

#[derive(new)]
pub struct CmdExecutor {
    command: String,
    executor: Arc<dyn PipedCmdExecutor>,
}

#[async_trait]
impl super::Component for CmdExecutor {
    type Output = Result<Exit>;

    async fn handle(&self) -> Self::Output {
        self.executor.piped_exec(self.command.as_str()).await
    }
}

#[derive(new)]
pub struct PrintableCmdNotFound<C> {
    command: String,
    inner: C,
}

#[async_trait]
impl<T: 'static, C: super::Component<Output = Result<T>> + Send + Sync> super::Component
    for PrintableCmdNotFound<C>
{
    type Output = Result<T>;

    async fn handle(&self) -> Self::Output {
        let result = self.inner.handle().await;

        match &result {
            Err(_) => eprintln!("retry: command not found '{}'", self.command),
            _ => (),
        };

        result
    }
}

pub struct WaitSec {
    sec: f64,
}

#[async_trait]
impl super::Component for WaitSec {
    type Output = ();

    async fn handle(&self) -> Self::Output {
        tokio::time::delay_for(tokio::time::Duration::from_secs_f64(self.sec)).await
    }
}

#[derive(new)]
pub struct SharedState<C> {
    config: Config,
    executor: Arc<dyn PipedCmdExecutor>,
    inner: C,
}

#[async_trait]
impl<T: 'static, C: super::Component<Output = T> + Send + Sync> super::Component
    for SharedState<C>
{
    type Output = T;

    async fn handle(&self) -> Self::Output {
        self.inner.handle().await
    }
}

impl From<SharedState<PrintableCmdNotFound<CmdExecutor>>> for SharedState<WaitSec> {
    fn from(state: SharedState<PrintableCmdNotFound<CmdExecutor>>) -> Self {
        Self {
            inner: WaitSec {
                sec: state.config.interval,
            },
            config: state.config,
            executor: state.executor,
        }
    }
}

impl From<SharedState<WaitSec>> for SharedState<PrintableCmdNotFound<CmdExecutor>> {
    fn from(state: SharedState<WaitSec>) -> Self {
        Self {
            inner: PrintableCmdNotFound {
                command: state.config.command.to_owned(),
                inner: CmdExecutor {
                    command: state.config.command.to_owned(),
                    executor: state.executor.clone(),
                },
            },
            config: state.config,
            executor: state.executor,
        }
    }
}
