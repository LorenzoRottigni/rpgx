use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use log::error;
use rpgx::common::coordinates::Coordinates;
use rpgx::common::direction::Direction;
use rpgx::prelude::Engine;

#[derive(Clone)]
pub enum Command {
    WalkTo(Coordinates),
    Step(Direction),
}

pub async fn sleep_ms(ms: u64) {
    #[cfg(feature = "web")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }

    #[cfg(feature = "desktop")]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    }
}

pub fn use_controller(engine: Signal<Engine>) -> Coroutine<Command> {
    use_coroutine({
        to_owned![engine];
        move |mut rx: UnboundedReceiver<Command>| async move {
            while let Some(command) = rx.next().await {
                let result: Result<(), Box<dyn std::error::Error>> = async {
                    match command {
                        Command::WalkTo(target) => {
                            let steps = engine
                                .read()
                                .map
                                .find_path(&engine.read().pawn.tile.pointer, &target);
                            match steps {
                                None => {
                                    error!("Path not found");
                                    return Err("Path not found".into());
                                }
                                Some(steps) => {
                                    for step in steps {
                                        sleep_ms(100).await;
                                        engine.write().move_to(step).map_err(|e| {
                                            Box::<dyn std::error::Error>::from(format!("{:?}", e))
                                        })?;
                                    }
                                    Ok(())
                                }
                            }
                        }
                        Command::Step(direction) => {
                            let mut engine_w = engine.write();
                            if let Ok(tile) = engine_w.step_to(direction) {
                                let action_ids = engine_w.map.get_actions_at(tile.pointer);
                                for action_id in action_ids {
                                    // Keep this as log only or handle as needed
                                    log::info!("Action triggered: {:?}", action_id);
                                }
                            }
                            Ok(())
                        }
                    }
                }
                .await;

                if let Err(e) = result {
                    error!("Movement error: {:?}", e);
                }
            }
        }
    })
}
