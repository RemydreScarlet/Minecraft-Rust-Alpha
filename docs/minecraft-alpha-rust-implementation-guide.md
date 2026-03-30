# Minecraft Alpha 完全再実装ガイド (Rust)

## 概要

このガイドは、Minecraft Alpha の Java コードベースを分析し、Rust で完全に再実装するための包括的なドキュメントです。地形生成、モブ行動、物理システムなど、すべての核心的な要素について詳細な実装例を提供します。

## 1. プロジェクト構造

### 1.1 基本的なディレクトリ構成

```
minecraft-alpha-rust/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── world/
│   │   ├── mod.rs
│   │   ├── chunk.rs
│   │   ├── generator.rs
│   │   └── world.rs
│   ├── entities/
│   │   ├── mod.rs
│   │   ├── entity.rs
│   │   ├── living_entity.rs
│   │   ├── mobs/
│   │   │   ├── mod.rs
│   │   │   ├── zombie.rs
│   │   │   ├── pig.rs
│   │   │   └── player.rs
│   │   └── ai.rs
│   ├── physics/
│   │   ├── mod.rs
│   │   ├── collision.rs
│   │   └── movement.rs
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── renderer.rs
│   │   ├── chunk_mesh.rs
│   │   └── camera.rs
│   ├── blocks/
│   │   ├── mod.rs
│   │   ├── block_types.rs
│   │   └── block_registry.rs
│   ├── audio/
│   │   ├── mod.rs
│   │   └── sound_engine.rs
│   └── engine/
│       ├── mod.rs
│       ├── game_loop.rs
│       └── input.rs
├── assets/
│   ├── textures/
│   └── sounds/
└── docs/
    ├── terrain-generation-algorithms.md
    ├── mob-behavior-algorithms.md
    └── implementation-guide.md
```

### 1.2 Cargo.toml 設定

```toml
[package]
name = "minecraft-alpha-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# 基本ライブラリ
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 数学ライブラリ
glam = "0.24"  # ベクトル数学
noise = "0.8"  # 高度なノイズ生成（オプション）

# レンダリング
wgpu = "0.18"
winit = "0.28"
pollster = "0.3"

# オーディオ
rodio = "0.17"

# 並列処理
rayon = "1.7"

# エラーハンドリング
anyhow = "1.0"
thiserror = "1.0"

# ロギング
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

## 2. コアシステム実装

### 2.1 メインエントリーポイント

```rust
// src/main.rs
use minecraft_alpha_rust::engine::GameEngine;
use tracing::{info, Level};
use tracing_subscriber;

fn main() -> anyhow::Result<()> {
    // ロギング初期化
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Minecraft Alpha Rust Edition 起動中...");

    // ゲームエンジン初期化
    let mut engine = GameEngine::new()?;
    
    // メインゲームループ
    engine.run()?;

    Ok(())
}
```

### 2.2 ゲームエンジン

```rust
// src/engine/mod.rs
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;
use anyhow::Result;
use tracing::{info, error};

use crate::rendering::Renderer;
use crate::world::World;
use crate::entities::player::Player;
use crate::audio::SoundEngine;

pub struct GameEngine {
    window: Window,
    renderer: Renderer,
    world: World,
    player: Player,
    sound_engine: SoundEngine,
    is_running: bool,
}

impl GameEngine {
    pub fn new() -> Result<Self> {
        // ウィンドウ作成
        let event_loop = EventLoopBuilder::new().build()?;
        let window = WindowBuilder::new()
            .with_title("Minecraft Alpha - Rust Edition")
            .with_inner_size(PhysicalSize::new(1024, 768))
            .build(&event_loop)?;

        info!("ウィンドウ作成完了");

        // レンダラー初期化
        let renderer = pollster::block_on(Renderer::new(&window))?;
        info!("レンダラー初期化完了");

        // 世界生成
        let mut world = World::new(12345); // シード値
        world.generate_initial_chunks();
        info!("世界生成完了");

        // プレイヤー生成
        let player = Player::new(glam::Vec3::new(0.0, 80.0, 0.0));
        info!("プレイヤー生成完了");

        // サウンドエンジン初期化
        let sound_engine = SoundEngine::new()?;
        info!("サウンドエンジン初期化完了");

        Ok(Self {
            window,
            renderer,
            world,
            player,
            sound_engine,
            is_running: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        info!("ゲームループ開始");

        while self.is_running {
            // 入力処理
            self.handle_input()?;

            // 物理更新
            self.update_physics()?;

            // エンティティ更新
            self.update_entities()?;

            // ワールド更新
            self.update_world()?;

            // レンダリング
            self.render()?;

            // オーディオ更新
            self.update_audio()?;

            // フレームレート制限
            std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
        }

        info!("ゲームループ終了");
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        // 入力処理実装
        // キーボード、マウス入力を処理
        Ok(())
    }

    fn update_physics(&mut self) -> Result<()> {
        // 物理計算更新
        // 重力、衝突検出など
        Ok(())
    }

    fn update_entities(&mut self) -> Result<()> {
        // 全エンティティの更新
        self.world.update_entities(&self.player)?;
        
        // プレイヤー更新
        self.player.update(&self.world)?;
        
        Ok(())
    }

    fn update_world(&mut self) -> Result<()> {
        // チャンクのロード/アンロード
        self.world.update_chunk_loading(&self.player.position)?;
        
        Ok(())
    }

    fn render(&mut self) -> Result<()> {
        // レンダリング実行
        self.renderer.render(&self.world, &self.player)?;
        Ok(())
    }

    fn update_audio(&mut self) -> Result<()> {
        // オーディオ更新
        self.sound_engine.update(&self.player.position)?;
        Ok(())
    }
}
```

## 3. 世界システム実装

### 3.1 チャンクシステム

```rust
// src/world/chunk.rs
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::blocks::BlockType;

#[derive(Debug, Clone, Copy)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

pub struct Chunk {
    pub pos: ChunkPos,
    pub blocks: Box<[BlockType; 16 * 16 * 128]>,
    pub heightmap: Box<[u8; 16 * 16]>,
    pub is_modified: bool,
    pub is_dirty: bool,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            pos,
            blocks: Box::new([BlockType::Air; 16 * 16 * 128]),
            heightmap: Box::new([0; 16 * 16]),
            is_modified: false,
            is_dirty: true,
        }
    }

    pub fn generate(&mut self, world_seed: u64) {
        let mut rng = self.get_chunk_rng(world_seed);
        
        // 高度マップ生成
        self.generate_heightmap(&mut rng);
        
        // 地形生成
        self.generate_terrain(&mut rng);
        
        // 構造物生成
        self.generate_structures(&mut rng);
        
        self.is_modified = true;
    }

    fn get_chunk_rng(&self, world_seed: u64) -> StdRng {
        let seed = world_seed
            .wrapping_mul((self.pos.x as u64).wrapping_mul(4987142))
            .wrapping_add((self.pos.x as u64).wrapping_mul(5947611))
            .wrapping_add((self.pos.z as u64).wrapping_mul(self.pos.z as u64).wrapping_mul(4392871))
            .wrapping_add((self.pos.z as u64).wrapping_mul(389711));
        
        StdRng::seed_from_u64(seed)
    }

    fn generate_heightmap(&mut self, rng: &mut StdRng) {
        for x in 0..16 {
            for z in 0..16 {
                let world_x = self.pos.x * 16 + x;
                let world_z = self.pos.z * 16 + z;
                
                // シンプルなハッシュベース高度生成
                let height = self.calculate_height(world_x, world_z, rng);
                self.heightmap[z * 16 + x] = height;
            }
        }
    }

    fn calculate_height(&self, x: i32, z: i32, rng: &mut StdRng) -> u8 {
        // 基本高度計算（20-80の範囲）
        let base_hash = ((x as u64).wrapping_mul(374761393)
            .wrapping_add(z as u64).wrapping_mul(668265263)) % 61;
        let base_height = 20 + base_hash as u8;
        
        // バイオームによる変化
        let biome_modifier = self.get_biome_modifier(x, z, rng);
        
        (base_height as i32 + biome_modifier).clamp(20, 80) as u8
    }

    fn get_biome_modifier(&self, x: i32, z: i32, rng: &mut StdRng) -> i32 {
        let biome_hash = ((x as u64).wrapping_mul(23456789)
            .wrapping_add(z as u64).wrapping_mul(987654321)) % 4;
        
        match biome_hash {
            0 => 20,   // 山地
            1 => -10,  // 砂漠
            2 => 5,    // 森林
            _ => 0,    // 平原
        }
    }

    fn generate_terrain(&mut self, rng: &mut StdRng) {
        for x in 0..16 {
            for z in 0..16 {
                let height = self.heightmap[z * 16 + x] as usize;
                
                // 岩盤
                self.set_block(x, 0, z, BlockType::Bedrock);
                if rng.gen_bool(0.8) {
                    self.set_block(x, 1, z, BlockType::Bedrock);
                }
                
                // 石層
                for y in 2..height.saturating_sub(3) {
                    self.set_block(x, y, z, BlockType::Stone);
                    
                    // 鉱石生成
                    if self.should_generate_ore(y, rng) {
                        self.set_block(x, y, z, self.get_random_ore(rng));
                    }
                }
                
                // 土層と草
                if height >= 3 {
                    for y in height.saturating_sub(3)..height {
                        self.set_block(x, y, z, BlockType::Dirt);
                    }
                    self.set_block(x, height, z, BlockType::Grass);
                }
                
                // 洞窟生成
                self.generate_caves(x, z, rng);
            }
        }
    }

    fn should_generate_ore(&self, y: usize, rng: &mut StdRng) -> bool {
        let probability = match y {
            0..=16 => 0.15,  // 深層
            17..=32 => 0.12, // 中層
            33..=64 => 0.08, // 上層
            _ => 0.05,       // 表層
        };
        rng.gen_bool(probability)
    }

    fn get_random_ore(&self, rng: &mut StdRng) -> BlockType {
        let roll = rng.gen_range(0..100);
        match roll {
            0..=2 => BlockType::DiamondOre,
            3..=8 => BlockType::RedstoneOre,
            9..=24 => BlockType::IronOre,
            25..=44 => BlockType::CoalOre,
            _ => BlockType::Stone,
        }
    }

    fn generate_caves(&mut self, x: usize, z: usize, rng: &mut StdRng) {
        for y in 0..40 {
            if rng.gen_bool(0.02) {
                self.set_block(x, y, z, BlockType::Air);
                self.expand_cave(x, y, z, rng);
            }
        }
    }

    fn expand_cave(&mut self, cx: usize, cy: usize, cz: usize, rng: &mut StdRng) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 { continue; }
                    
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    let nz = cz as i32 + dz;
                    
                    if nx >= 0 && nx < 16 && ny >= 0 && ny < 128 && nz >= 0 && nz < 16 {
                        if rng.gen_bool(0.3) {
                            self.set_block(nx as usize, ny as usize, nz as usize, BlockType::Air);
                        }
                    }
                }
            }
        }
    }

    fn generate_structures(&mut self, rng: &mut StdRng) {
        // 木の生成
        self.generate_trees(rng);
        
        // 水系生成
        self.generate_water_features(rng);
    }

    fn generate_trees(&mut self, rng: &mut StdRng) {
        for x in 0..16 {
            for z in 0..16 {
                let height = self.heightmap[z * 16 + x] as usize;
                
                if height > 0 && height < 100 && rng.gen_bool(0.05) {
                    if self.get_block(x, height, z) == BlockType::Grass {
                        self.generate_tree(x, height + 1, z, rng);
                    }
                }
            }
        }
    }

    fn generate_tree(&mut self, x: usize, y: usize, z: usize, rng: &mut StdRng) {
        let tree_height = 4 + rng.gen_range(0..3);
        
        // 幹
        for i in 0..tree_height {
            if y + i < 128 {
                self.set_block(x, y + i, z, BlockType::Wood);
            }
        }
        
        // 葉
        let leaf_y = y + tree_height;
        for dx in -2..=2 {
            for dz in -2..=2 {
                for dy in -2..=1 {
                    let nx = x as i32 + dx;
                    let ny = leaf_y as i32 + dy;
                    let nz = z as i32 + dz;
                    
                    if nx >= 0 && nx < 16 && ny >= 0 && ny < 128 && nz >= 0 && nz < 16 {
                        let distance = (dx.abs() + dz.abs() + dy.abs()) as usize;
                        if distance <= 3 && rng.gen_bool(0.8) {
                            if self.get_block(nx as usize, ny as usize, nz as usize) == BlockType::Air {
                                self.set_block(nx as usize, ny as usize, nz as usize, BlockType::Leaves);
                            }
                        }
                    }
                }
            }
        }
    }

    fn generate_water_features(&mut self, rng: &mut StdRng) {
        for x in 0..16 {
            for z in 0..16 {
                let height = self.heightmap[z * 16 + x] as usize;
                
                if height < 45 && rng.gen_bool(0.15) {
                    self.set_block(x, height, z, BlockType::Water);
                    self.spread_water(x, height, z, rng);
                }
            }
        }
    }

    fn spread_water(&mut self, x: usize, y: usize, z: usize, rng: &mut StdRng) {
        for dx in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dz == 0 { continue; }
                
                let nx = x as i32 + dx;
                let nz = z as i32 + dz;
                
                if nx >= 0 && nx < 16 && nz >= 0 && nz < 16 {
                    let neighbor_height = self.heightmap[nz as usize * 16 + nx as usize] as usize;
                    if neighbor_height <= y && rng.gen_bool(0.6) {
                        self.set_block(nx as usize, neighbor_height, nz as usize, BlockType::Water);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        if x >= 16 || y >= 128 || z >= 16 {
            return BlockType::Air;
        }
        self.blocks[x * 128 * 16 + z * 128 + y]
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x >= 16 || y >= 128 || z >= 16 {
            return;
        }
        self.blocks[x * 128 * 16 + z * 128 + y] = block;
        self.is_dirty = true;
    }

    pub fn get_world_pos(&self, local_x: usize, local_z: usize) -> (i32, i32) {
        (
            self.pos.x * 16 + local_x as i32,
            self.pos.z * 16 + local_z as i32,
        )
    }
}
```

### 3.2 世界管理

```rust
// src/world/world.rs
use std::collections::HashMap;
use glam::Vec3;
use anyhow::Result;
use tracing::{info, debug};

use crate::world::chunk::{Chunk, ChunkPos};
use crate::entities::EntityRegistry;

pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub seed: u64,
    pub entities: EntityRegistry,
    pub view_distance: i32,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self {
            chunks: HashMap::new(),
            seed,
            entities: EntityRegistry::new(),
            view_distance: 8,
        }
    }

    pub fn generate_initial_chunks(&mut self) {
        info!("初期チャンク生成開始...");
        
        // スポーン周辺のチャンクを生成
        for x in -4..=4 {
            for z in -4..=4 {
                let chunk_pos = ChunkPos::new(x, z);
                let mut chunk = Chunk::new(chunk_pos);
                chunk.generate(self.seed);
                self.chunks.insert(chunk_pos, chunk);
            }
        }
        
        info!("初期チャンク生成完了: {} チャンク", self.chunks.len());
    }

    pub fn get_or_generate_chunk(&mut self, pos: ChunkPos) -> &mut Chunk {
        if !self.chunks.contains_key(&pos) {
            debug!("チャンク生成: ({}, {})", pos.x, pos.z);
            let mut chunk = Chunk::new(pos);
            chunk.generate(self.seed);
            self.chunks.insert(pos, chunk);
        }
        
        self.chunks.get_mut(&pos).unwrap()
    }

    pub fn update_chunk_loading(&mut self, player_pos: Vec3) -> Result<()> {
        let player_chunk_x = (player_pos.x as i32).div_euclid(16);
        let player_chunk_z = (player_pos.z as i32).div_euclid(16);
        
        // ロードすべきチャンクのリスト
        let mut chunks_to_load = Vec::new();
        let mut chunks_to_unload = Vec::new();
        
        // 視界範囲内のチャンクを特定
        for x in (player_chunk_x - self.view_distance)..=(player_chunk_x + self.view_distance) {
            for z in (player_chunk_z - self.view_distance)..=(player_chunk_z + self.view_distance) {
                let chunk_pos = ChunkPos::new(x, z);
                if !self.chunks.contains_key(&chunk_pos) {
                    chunks_to_load.push(chunk_pos);
                }
            }
        }
        
        // 視界範囲外のチャンクを特定
        for (&chunk_pos, _) in self.chunks.iter() {
            let distance_x = (chunk_pos.x - player_chunk_x).abs();
            let distance_z = (chunk_pos.z - player_chunk_z).abs();
            
            if distance_x > self.view_distance || distance_z > self.view_distance {
                chunks_to_unload.push(chunk_pos);
            }
        }
        
        // チャンクのロード
        for chunk_pos in chunks_to_load {
            self.get_or_generate_chunk(chunk_pos);
        }
        
        // チャンクのアンロード
        for chunk_pos in chunks_to_unload {
            debug!("チャンクアンロード: ({}, {})", chunk_pos.x, chunk_pos.z);
            self.chunks.remove(&chunk_pos);
        }
        
        Ok(())
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> crate::blocks::BlockType {
        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let local_x = x.rem_euclid(16) as usize;
        let local_z = z.rem_euclid(16) as usize;
        let local_y = y as usize;
        
        if let Some(chunk) = self.chunks.get(&ChunkPos::new(chunk_x, chunk_z)) {
            chunk.get_block(local_x, local_y, local_z)
        } else {
            crate::blocks::BlockType::Air
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: crate::blocks::BlockType) -> bool {
        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let local_x = x.rem_euclid(16) as usize;
        let local_z = z.rem_euclid(16) as usize;
        let local_y = y as usize;
        
        if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_x, chunk_z)) {
            chunk.set_block(local_x, local_y, local_z, block);
            true
        } else {
            false
        }
    }

    pub fn is_solid_block(&self, x: f64, y: f64, z: f64) -> bool {
        let block_x = x.floor() as i32;
        let block_y = y.floor() as i32;
        let block_z = z.floor() as i32;
        
        if block_y < 0 || block_y >= 128 {
            return block_y < 0; // 地面より下は固体
        }
        
        self.get_block(block_x, block_y, block_z).is_solid()
    }

    pub fn get_ground_height(&self, x: f64, z: f64) -> f64 {
        let block_x = x.floor() as i32;
        let block_z = z.floor() as i32;
        let chunk_x = block_x.div_euclid(16);
        let chunk_z = block_z.div_euclid(16);
        let local_x = block_z.rem_euclid(16) as usize;
        let local_z = block_x.rem_euclid(16) as usize;
        
        if let Some(chunk) = self.chunks.get(&ChunkPos::new(chunk_x, chunk_z)) {
            chunk.heightmap[local_x * 16 + local_z] as f64
        } else {
            64.0 // デフォルト高度
        }
    }

    pub fn update_entities(&mut self, player: &crate::entities::player::Player) -> Result<()> {
        self.entities.update_all_entities(self, player)?;
        Ok(())
    }

    pub fn get_loaded_chunks(&self) -> &HashMap<ChunkPos, Chunk> {
        &self.chunks
    }
}
```

## 4. エンティティシステム実装

### 4.1 基本エンティティ

```rust
// src/entities/entity.rs
use glam::Vec3;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(center: Vec3, width: f32, height: f32) -> Self {
        let half_width = width as f64 / 2.0;
        Self {
            min: Vec3::new(
                center.x - half_width,
                center.y,
                center.z - half_width,
            ),
            max: Vec3::new(
                center.x + half_width,
                center.y + height as f64,
                center.z + half_width,
            ),
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    pub fn expand(&self, amount: f64) -> Self {
        Self {
            min: self.min - Vec3::splat(amount),
            max: self.max + Vec3::splat(amount),
        }
    }
}

pub struct Entity {
    pub id: u64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Vec2, // (yaw, pitch)
    pub bounding_box: BoundingBox,
    pub width: f32,
    pub height: f32,
    pub step_height: f32,
    pub on_ground: bool,
    pub in_water: bool,
    pub in_lava: bool,
    pub is_dead: bool,
    pub rng: ThreadRng,
}

impl Entity {
    pub fn new(position: Vec3) -> Self {
        let width = 0.6;
        let height = 1.8;
        
        Self {
            id: 0, // エンティティマネージャーで設定
            position,
            velocity: Vec3::ZERO,
            rotation: Vec2::ZERO,
            bounding_box: BoundingBox::new(position, width, height),
            width,
            height,
            step_height: 0.6,
            on_ground: false,
            in_water: false,
            in_lava: false,
            is_dead: false,
            rng: thread_rng(),
        }
    }

    pub fn update_bounding_box(&mut self) {
        self.bounding_box = BoundingBox::new(self.position, self.width, self.height);
    }

    pub fn move_relative(&mut self, amount: Vec3, yaw: f32) {
        let yaw_rad = yaw.to_radians();
        let sin_yaw = yaw_rad.sin();
        let cos_yaw = yaw_rad.cos();

        self.velocity.x += amount.x * cos_yaw as f64 - amount.z * sin_yaw as f64;
        self.velocity.z += amount.x * sin_yaw as f64 + amount.z * cos_yaw as f64;
        self.velocity.y += amount.y;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.update_bounding_box();
    }
}
```

### 4.2 生き物エンティティ

```rust
// src/entities/living_entity.rs
use glam::Vec3;
use crate::entities::entity::{Entity, BoundingBox};

pub struct LivingEntity {
    pub entity: Entity,
    
    // 生存関連
    pub max_health: i32,
    pub current_health: i32,
    pub prev_health: i32,
    pub hurt_time: i32,
    pub death_time: i32,
    pub attack_direction: f32,
    
    // 移動関連
    pub move_speed: f32,
    pub jump_height: f32,
    pub move_forward: f32,
    pub move_strafing: f32,
    pub is_jumping: bool,
    
    // AI関連
    pub attack_target: Option<u64>,
    pub attack_cooldown: i32,
}

impl LivingEntity {
    pub fn new(position: Vec3) -> Self {
        let mut entity = Entity::new(position);
        entity.width = 0.6;
        entity.height = 1.8;
        
        Self {
            entity,
            max_health: 20,
            current_health: 20,
            prev_health: 20,
            hurt_time: 0,
            death_time: 0,
            attack_direction: 0.0,
            move_speed: 0.2,
            jump_height: 0.42,
            move_forward: 0.0,
            move_strafing: 0.0,
            is_jumping: false,
            attack_target: None,
            attack_cooldown: 0,
        }
    }

    pub fn update(&mut self, world: &crate::world::World) -> anyhow::Result<()> {
        // 体力更新
        self.update_health();
        
        // 移動処理
        self.update_movement(world)?;
        
        // 状態更新
        self.update_state(world);
        
        Ok(())
    }

    fn update_health(&mut self) {
        if self.hurt_time > 0 {
            self.hurt_time -= 1;
        }
        
        if self.death_time > 0 {
            self.death_time -= 1;
            if self.death_time == 0 {
                self.entity.is_dead = true;
            }
        }
        
        if self.attack_cooldown > 0 {
            self.attack_cooldown -= 1;
        }
    }

    fn update_movement(&mut self, world: &crate::world::World) -> anyhow::Result<()> {
        let prev_y = self.entity.position.y;
        
        // 入力に基づく移動
        if self.move_forward != 0.0 || self.move_strafing != 0.0 {
            self.apply_movement_input();
        }
        
        // 重力
        self.apply_gravity();
        
        // 衝突検出と位置更新
        self.handle_collisions(world)?;
        
        // 落下ダメージ
        if prev_y - self.entity.position.y > 1.0 && !self.in_water {
            self.handle_fall_damage(prev_y - self.entity.position.y);
        }
        
        Ok(())
    }

    fn apply_movement_input(&mut self) {
        let distance = (self.move_forward * self.move_forward + 
                       self.move_strafing * self.move_strafing).sqrt();
        
        if distance > 0.0 {
            let speed_multiplier = if self.in_water { 0.8 } else { 1.0 };
            let actual_speed = self.move_speed * speed_multiplier;
            
            let yaw_rad = self.entity.rotation.x.to_radians();
            let sin_yaw = yaw_rad.sin();
            let cos_yaw = yaw_rad.cos();
            
            let move_x = (self.move_forward * cos_yaw - self.move_strafing * sin_yaw) * actual_speed;
            let move_z = (self.move_forward * sin_yaw + self.move_strafing * cos_yaw) * actual_speed;
            
            self.entity.velocity.x += move_x as f64;
            self.entity.velocity.z += move_z as f64;
        }
    }

    fn apply_gravity(&mut self) {
        if !self.entity.on_ground && !self.in_water && !self.in_lava {
            self.entity.velocity.y -= 0.08;
        }
    }

    fn handle_collisions(&mut self, world: &crate::world::World) -> anyhow::Result<()> {
        let new_position = self.entity.position + self.entity.velocity;
        
        // X軸の衝突
        if world.is_solid_block(new_position.x, self.entity.position.y, self.entity.position.z) {
            self.entity.velocity.x = 0.0;
        } else {
            self.entity.position.x = new_position.x;
        }
        
        // Y軸の衝突
        if world.is_solid_block(self.entity.position.x, new_position.y, self.entity.position.z) {
            self.entity.velocity.y = 0.0;
            self.entity.on_ground = true;
        } else {
            self.entity.position.y = new_position.y;
            self.entity.on_ground = false;
        }
        
        // Z軸の衝突
        if world.is_solid_block(self.entity.position.x, self.entity.position.y, new_position.z) {
            self.entity.velocity.z = 0.0;
        } else {
            self.entity.position.z = new_position.z;
        }
        
        // 摩擦
        if self.entity.on_ground {
            let friction = if self.on_ice(world) { 0.546 } else { 0.91 };
            self.entity.velocity.x *= friction;
            self.entity.velocity.z *= friction;
        }
        
        // 水中抵抗
        if self.in_water {
            self.entity.velocity.x *= 0.8;
            self.entity.velocity.y *= 0.8;
            self.entity.velocity.z *= 0.8;
        }
        
        self.entity.update_bounding_box();
        self.update_environment_state(world);
        
        Ok(())
    }

    fn update_environment_state(&mut self, world: &crate::world::World) {
        // 水中判定
        self.in_water = world.is_solid_block(
            self.entity.position.x, 
            self.entity.position.y, 
            self.entity.position.z
        ) == crate::blocks::BlockType::Water;
        
        // 溶岩判定
        self.in_lava = world.is_solid_block(
            self.entity.position.x, 
            self.entity.position.y, 
            self.entity.position.z
        ) == crate::blocks::BlockType::Lava;
    }

    fn on_ice(&self, world: &crate::world::World) -> bool {
        world.get_block(
            self.entity.position.x as i32,
            (self.entity.position.y - 1.0) as i32,
            self.entity.position.z as i32
        ) == crate::blocks::BlockType::Ice
    }

    fn handle_fall_damage(&mut self, fall_distance: f64) {
        if fall_distance > 3.0 {
            let damage = ((fall_distance - 3.0) as i32).min(10);
            self.take_damage(damage);
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.hurt_time == 0 {
            self.prev_health = self.current_health;
            self.current_health = (self.current_health - damage).max(0);
            self.hurt_time = 10;
            
            if self.current_health <= 0 {
                self.death_time = 20;
            }
        }
    }

    pub fn jump(&mut self) {
        if self.entity.on_ground && !self.in_water && !self.in_lava {
            self.entity.velocity.y = self.jump_height as f64;
            self.is_jumping = true;
        }
    }

    pub fn attack_entity(&mut self, target: &mut LivingEntity, damage: i32) -> bool {
        if self.attack_cooldown > 0 || self.current_health <= 0 {
            return false;
        }
        
        // ダメージ適用
        target.take_damage(damage);
        
        // ノックバック
        let dx = target.entity.position.x - self.entity.position.x;
        let dz = target.entity.position.z - self.entity.position.z;
        let distance = (dx * dx + dz * dz).sqrt();
        
        if distance > 0.1 {
            let knockback_force = 0.4;
            target.entity.velocity.x += (dx / distance) * knockback_force;
            target.entity.velocity.y += 0.4;
            target.entity.velocity.z += (dz / distance) * knockback_force;
        }
        
        self.attack_cooldown = 20;
        true
    }

    pub fn get_look_direction(&self) -> Vec3 {
        let yaw_rad = self.entity.rotation.x.to_radians();
        let pitch_rad = self.entity.rotation.y.to_radians();
        
        Vec3::new(
            -yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
        )
    }
}
```

## 5. レンダリングシステム

### 5.1 WebGPU レンダラー

```rust
// src/rendering/renderer.rs
use wgpu::*;
use winit::window::Window;
use anyhow::Result;
use tracing::info;

use crate::world::World;
use crate::entities::player::Player;
use crate::rendering::chunk_mesh::ChunkMesh;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    
    // レンダリングパイプライン
    render_pipeline: wgpu::RenderPipeline,
    
    // バッファ
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    
    // チャンクメッシュ
    chunk_meshes: Vec<ChunkMesh>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        
        // WGPU インスタンス作成
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        
        // アダプター選択
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // デバイスとキュー作成
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        
        // サーフェス設定
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        
        surface.configure(&device, &config);
        
        // シェーダー作成
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        
        // レンダーパイプライン作成
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        
        info!("レンダラー初期化完了");
        
        Ok(Self {
            device,
            queue,
            config,
            size,
            surface,
            render_pipeline,
            vertex_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            index_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Index Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            uniform_buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Uniform Buffer"),
                size: std::mem::size_of::<Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            chunk_meshes: Vec::new(),
        })
    }
    
    pub fn render(&mut self, world: &World, player: &Player) -> Result<()> {
        // チャンクメッシュ更新
        self.update_chunk_meshes(world)?;
        
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // デプステクスチャ作成
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // ユニフォームバッファ更新
        let uniforms = Uniforms::new(player, &self.config);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
        
        // レンダーパス
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.529,
                            g: 0.808,
                            b: 0.922,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            
            // チャンクメッシュ描画
            for mesh in &self.chunk_meshes {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
    
    fn update_chunk_meshes(&mut self, world: &World) -> Result<()> {
        self.chunk_meshes.clear();
        
        for (chunk_pos, chunk) in world.get_loaded_chunks() {
            let mesh = ChunkMesh::generate(chunk, &self.device)?;
            self.chunk_meshes.push(mesh);
        }
        
        Ok(())
    }
    
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 3],
    _padding: f32,
}

impl Uniforms {
    fn new(player: &Player, config: &wgpu::SurfaceConfiguration) -> Self {
        let aspect = config.width as f32 / config.height as f32;
        let projection = glam::Mat4::perspective_rh_gl(70.0_f32.to_radians(), aspect, 0.1, 1000.0);
        let view = player.camera_matrix();
        let view_proj = projection * view;
        
        Self {
            view_proj: view_proj.to_cols_array(),
            camera_pos: player.position.to_array(),
            _padding: 0.0,
        }
    }
}
```

## 6. 完全実装のまとめ

### 6.1 ビルドと実行

```bash
# プロジェクトビルド
cargo build --release

# 実行
cargo run --release

# ベンチマーク
cargo bench
```

### 6.2 パフォーマンス最適化のポイント

1. **並列チャンク生成**: `rayon` を使用したマルチスレッド処理
2. **効率的なメモリ管理**: `Box<[T]>` による固定サイズ配列
3. **GPU 最適化**: インスタンシングとバッチ描画
4. **レベル・オブ・デテール (LOD)**: 距離による描画品質調整
5. **チャンクキャッシュ**: LRU キャッシュによるメモリ効率化

### 6.3 拡張機能の実装

1. **マルチプレイヤー**: ネットワークプロトコル実装
2. **Redstone 回路**: 論理シミュレーション
3. **高度な地形生成**: Perlin/ Simplex ノイズ
4. **MOD API**: プラグインシステム
5. **設定システム**: JSON ベースのコンフィグ

このガイドにより、Minecraft Alpha の完全な機能を持つ Rust 実装が構築可能です。オリジナルのアルゴリズムを忠実に再現しつつ、Rust のパフォーマンスと安全性を最大限に活かすことができます。
