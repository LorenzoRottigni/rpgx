use std::any::Any;

use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use log::error;
use rpgx::library::Library;
use rpgx::prelude::Engine;
use rpgx::prelude::{Coordinates, Direction};
use rpgx::traits::Grid;

#[derive(Clone, Debug)]
pub enum Command {
    WalkTo(Coordinates),
    Step(Direction),
}

pub async fn sleep_ms(_ms: u64) {
    #[cfg(feature = "web")]
    {
        gloo_timers::future::TimeoutFuture::new(_ms as u32).await;
    }

    #[cfg(feature = "desktop")]
    {
        tokio::time::sleep(std::time::Duration::from_millis(_ms)).await;
    }
}

pub fn use_controller(
    engine: Signal<Engine>,
    library: Signal<Library<Box<dyn Any>>>,
) -> Coroutine<Command> {
    use_coroutine({
        to_owned![engine];
        move |mut rx: UnboundedReceiver<Command>| async move {
            while let Some(command) = rx.next().await {
                let result: Result<(), Box<dyn std::error::Error>> = async {
                    match command {
                        Command::WalkTo(target) => {
                            let steps = engine.read().get_active_scene().unwrap().map.find_path(
                                &engine
                                    .read()
                                    .get_active_scene()
                                    .unwrap()
                                    .pawn
                                    .as_ref()
                                    .unwrap()
                                    .pointer,
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
                            if let Ok(pointer) =
                                _engine.get_active_scene_mut().unwrap().step_to(direction)
                            {
                                _engine
                                    .get_active_scene()
                                    .unwrap()
                                    .map
                                    .get_actions_at(pointer)
                                    .into_iter()
                                    .for_each(|action_id| {
                                        if let Some(boxed) = library.read().get_by_id(action_id) {
                                            if let Some(unboxed) =
                                                boxed.downcast_ref::<Box<dyn Fn(&mut Engine)>>()
                                            {
                                                println!("calling unboxed action");
                                                unboxed(&mut _engine)
                                            }
                                        }
                                    });
                                // for action_id in action_ids {
                                //     // Keep this as log only or handle as needed
                                //     log::info!("Action triggered: {:?}", action_id);
                                // }
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
