//! Thread communication and synchronization utilities
//! 
//! This module provides the foundation for multithreaded game loop architecture,
//! including message passing, thread-safe containers, and coordination primitives.

use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::world::world::World;
use crate::math::position::{WorldPos, ChunkPos};

/// Messages sent between game threads
#[derive(Debug, Clone)]
pub enum GameMessage {
    /// World update tick
    WorldTick,
    /// Request world state for rendering
    RequestWorldState,
    /// Set block at position
    SetBlock { pos: WorldPos, block_id: u8 },
    /// Add chunk to world
    AddChunk { chunk_pos: ChunkPos },
    /// Remove chunk from world
    RemoveChunk { chunk_pos: ChunkPos },
    /// Shutdown all threads
    Shutdown,
}

/// Messages sent from world thread back to main thread
#[derive(Debug, Clone)]
pub enum WorldResponse {
    /// World state update completed
    TickCompleted { tick_count: u64, partial_ticks: f32 },
    /// World state snapshot for rendering
    WorldStateSnapshot { world_data: WorldSnapshot },
    /// Block operation result
    BlockOperationResult { success: bool },
}

/// Thread-safe snapshot of world state for rendering
#[derive(Debug, Clone)]
pub struct WorldSnapshot {
    pub tick_count: u64,
    pub time: u64,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub partial_ticks: f32,
    // Note: For performance, we don't include full chunk/entity data here
    // Rendering thread will request specific data as needed
}

impl WorldSnapshot {
    pub fn from_world(world: &World, tick_count: u64, partial_ticks: f32) -> Self {
        Self {
            tick_count,
            time: world.time,
            spawn_x: world.spawn_x,
            spawn_y: world.spawn_y,
            spawn_z: world.spawn_z,
            partial_ticks,
        }
    }
}

/// Thread-safe world state container
pub struct ThreadSafeWorld {
    world: Arc<Mutex<World>>,
    is_running: Arc<AtomicBool>,
    tick_counter: Arc<AtomicU64>,
}

impl ThreadSafeWorld {
    pub fn new(world: World) -> Self {
        Self {
            world: Arc::new(Mutex::new(world)),
            is_running: Arc::new(AtomicBool::new(true)),
            tick_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_world(&self) -> Arc<Mutex<World>> {
        Arc::clone(&self.world)
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn shutdown(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn get_tick_count(&self) -> u64 {
        self.tick_counter.load(Ordering::Relaxed)
    }

    pub fn increment_tick(&self) {
        self.tick_counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// Thread manager for coordinating game threads
pub struct ThreadManager {
    world_thread_handle: Option<JoinHandle<()>>,
    world_sender: Option<Sender<GameMessage>>,
    world_receiver: Option<Receiver<WorldResponse>>,
    thread_safe_world: ThreadSafeWorld,
}

impl ThreadManager {
    pub fn new(world: World) -> Self {
        let thread_safe_world = ThreadSafeWorld::new(world);
        
        Self {
            world_thread_handle: None,
            world_sender: None,
            world_receiver: None,
            thread_safe_world,
        }
    }

    pub fn start_world_thread(&mut self) -> Result<(), Box<dyn std::any::Any + Send>> {
        let (world_sender, world_receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        
        let world = self.thread_safe_world.get_world();
        let is_running = Arc::clone(&self.thread_safe_world.is_running);
        let tick_counter = Arc::clone(&self.thread_safe_world.tick_counter);
        
        let handle = thread::spawn(move || {
            Self::world_thread_loop(world, world_receiver, response_sender, is_running, tick_counter);
        });

        self.world_thread_handle = Some(handle);
        self.world_sender = Some(world_sender);
        self.world_receiver = Some(response_receiver);

        Ok(())
    }

    fn world_thread_loop(
        world: Arc<Mutex<World>>,
        receiver: Receiver<GameMessage>,
        response_sender: Sender<WorldResponse>,
        is_running: Arc<AtomicBool>,
        tick_counter: Arc<AtomicU64>,
    ) {
        let target_tps = 20;
        let tick_duration = Duration::from_millis(1000 / target_tps);
        let mut last_tick = Instant::now();
        let mut partial_ticks = 0.0f32;

        while is_running.load(Ordering::Relaxed) {
            let now = Instant::now();
            let elapsed = now.duration_since(last_tick);
            
            // Calculate partial ticks for smooth interpolation
            let elapsed_ms = elapsed.as_millis() as f32;
            let expected_ticks = elapsed_ms / 50.0; // 50ms per tick at 20 TPS
            partial_ticks = expected_ticks.fract();

            // Process incoming messages
            let mut should_tick = false;
            
            match receiver.try_recv() {
                Ok(GameMessage::WorldTick) => {
                    should_tick = true;
                }
                Ok(GameMessage::RequestWorldState) => {
                    if let Ok(world_guard) = world.lock() {
                        let snapshot = WorldSnapshot::from_world(
                            &world_guard,
                            tick_counter.load(Ordering::Relaxed),
                            partial_ticks,
                        );
                        let _ = response_sender.send(WorldResponse::WorldStateSnapshot { world_data: snapshot });
                    }
                }
                Ok(GameMessage::SetBlock { pos, block_id }) => {
                    if let Ok(mut world_guard) = world.lock() {
                        let success = world_guard.set_block(pos, block_id);
                        let _ = response_sender.send(WorldResponse::BlockOperationResult { success });
                    }
                }
                Ok(GameMessage::Shutdown) => {
                    is_running.store(false, Ordering::Relaxed);
                    break;
                }
                Ok(_) => {}
                Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {
                    // No message, check if it's time for a tick
                    if elapsed >= tick_duration {
                        should_tick = true;
                    }
                }
            }

            // Perform world tick if needed
            if should_tick && elapsed >= tick_duration {
                if let Ok(mut world_guard) = world.lock() {
                    world_guard.update();
                    tick_counter.fetch_add(1, Ordering::Relaxed);
                    last_tick = now;

                    let _ = response_sender.send(WorldResponse::TickCompleted {
                        tick_count: tick_counter.load(Ordering::Relaxed),
                        partial_ticks,
                    });
                }
            }

            // Small sleep to prevent busy waiting
            if !should_tick && elapsed < tick_duration {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }

    pub fn send_world_message(&mut self, message: GameMessage) -> Result<(), mpsc::SendError<GameMessage>> {
        if let Some(ref sender) = self.world_sender {
            sender.send(message)
        } else {
            Err(mpsc::SendError(message))
        }
    }

    pub fn try_recv_world_response(&mut self) -> Result<WorldResponse, TryRecvError> {
        if let Some(ref receiver) = self.world_receiver {
            receiver.try_recv()
        } else {
            Err(TryRecvError::Disconnected)
        }
    }

    pub fn get_thread_safe_world(&self) -> &ThreadSafeWorld {
        &self.thread_safe_world
    }

    pub fn shutdown(&mut self) {
        self.thread_safe_world.shutdown();
        
        // Send shutdown message to world thread
        if let Some(ref sender) = self.world_sender {
            let _ = sender.send(GameMessage::Shutdown);
        }

        // Wait for world thread to finish
        if let Some(handle) = self.world_thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for ThreadManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}
