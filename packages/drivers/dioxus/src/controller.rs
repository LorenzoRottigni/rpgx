use crate::utils::sleep_ms;
use crate::{Direction, EngineHandle, MapError, Tile, TilePointer}; // or adjust based on your module tree
use dioxus::prelude::*;
use futures_util::stream::StreamExt; // if you use that from somewhere

#[derive(Debug)]
pub enum Command {
    WalkTo(TilePointer),
    Step(Direction),
}

pub fn use_controller(props: GridProps) -> UseCoroutineHandle<Command> {
    let mut engine = props.engine.clone();
    let library = props.library.clone();

    use_coroutine({
        to_owned![engine, library];
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
                                None => Err("Path not found".into()),
                                Some(steps) => {
                                    for step in steps {
                                        sleep_ms(100).await;
                                        engine
                                            .write()
                                            .move_to(step)
                                            .map_err(|e| format!("{:?}", e).into())?;
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
                                    if let Some(action) = library.read().get_action_by_id(action_id)
                                    {
                                        action();
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                }
                .await;

                if let Err(e) = result {
                    error!("Controller error: {:?}", e);
                }
            }
        }
    })
}
