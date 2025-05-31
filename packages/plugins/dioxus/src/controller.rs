use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use log::error;
use rpgx::common::coordinates::Coordinates;
use rpgx::common::direction::Direction;
use rpgx::prelude::Engine;
use web_sys::console;

#[derive(Clone, Debug)]
pub enum Command {
    WalkTo(Coordinates),
    Step(Direction),
}

pub async fn sleep_ms(ms: u64) {
    #[cfg(feature = "web")]
    {
        console::log_1(&"sleep ms web".into());
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
                console::log_1(&format!("received command: {:?}", command).into());
                let result: Result<(), Box<dyn std::error::Error>> = async {
                    match command {
                        Command::WalkTo(target) => {
                            let steps = engine.read().get_active_scene().unwrap().map.find_path(
                                &engine.read().get_active_scene().unwrap().pawn.tile.pointer,
                                &target,
                            );
                            match steps {
                                None => {
                                    error!("Path not found");
                                    return Err("Path not found".into());
                                }
                                Some(steps) => {
                                    for step in steps {
                                        sleep_ms(100).await;
                                        engine
                                            .write()
                                            .get_active_scene_mut()
                                            .unwrap()
                                            .move_to(step)
                                            .map_err(|e| {
                                                Box::<dyn std::error::Error>::from(format!(
                                                    "{:?}",
                                                    e
                                                ))
                                            })?;
                                    }
                                    Ok(())
                                }
                            }
                        }
                        Command::Step(direction) => {
                            let mut _engine = engine.write();
                            if let Ok(tile) =
                                _engine.get_active_scene_mut().unwrap().step_to(direction)
                            {
                                let action_ids = _engine
                                    .get_active_scene()
                                    .unwrap()
                                    .map
                                    .get_actions_at(tile.pointer);
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
