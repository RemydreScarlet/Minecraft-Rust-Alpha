# Rust移植設計パターン

## 概要

Minecraft Alpha v1.1.2_01をRustに移植する際の設計パターンとアーキテクチャガイドライン。Javaのオブジェクト指向からRustの所有権システムへの変換を考慮した設計を提案します。

## 基本設計原則

### 1. 所有権と借用チェッカーの活用
```rust
// 複数の参照を必要とする場合
pub struct Game {
    world: Arc<RwLock<World>>,
    renderer: Arc<Mutex<Renderer>>,
    player: Arc<Mutex<Player>>,
}

// スレッドセーフな共有
impl Game {
    pub fn update(&self, delta_time: f32) {
        let world = self.world.read().unwrap();
        let mut renderer = self.renderer.lock().unwrap();
        
        renderer.render_world(&*world, delta_time);
    }
}
```

### 2. トレイトによる抽象化
```rust
// 共通インターフェースの定義
pub trait Block {
    fn get_id(&self) -> u8;
    fn get_properties(&self) -> BlockProperties;
    fn on_place(&self, world: &mut World, pos: BlockPos);
    fn on_break(&self, world: &mut World, pos: BlockPos);
}

// 動的ディスパッチ
pub struct BlockRegistry {
    blocks: [Option<Box<dyn Block + Send + Sync>>; 256],
}
```

### 3. エラーハンドリング
```rust
#[derive(Debug)]
pub enum GameError {
    WorldError(WorldError),
    RenderError(RenderError),
    NetworkError(NetworkError),
    IoError(std::io::Error),
}

pub type GameResult<T> = Result<T, GameError>;

// エラー伝播
impl From<WorldError> for GameError {
    fn from(err: WorldError) -> Self {
        GameError::WorldError(err)
    }
}
```

## コアシステム設計

### ゲームループ
```rust
pub struct Game {
    window: Window,
    renderer: Renderer,
    world: Arc<RwLock<World>>,
    input_manager: InputManager,
    ui_manager: GuiManager,
    network_manager: Option<NetworkManager>,
    running: bool,
}

impl Game {
    pub fn new() -> GameResult<Self> {
        let window = Window::new("Minecraft Alpha", 854, 480)?;
        let renderer = Renderer::new(&window)?;
        let world = Arc::new(RwLock::new(World::new()));
        
        Ok(Self {
            window,
            renderer,
            world,
            input_manager: InputManager::new(),
            ui_manager: GuiManager::new(),
            network_manager: None,
            running: true,
        })
    }
    
    pub fn run(&mut self) -> GameResult<()> {
        let mut last_time = Instant::now();
        
        while self.running {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_time).as_secs_f32();
            last_time = current_time;
            
            self.handle_events()?;
            self.update(delta_time)?;
            self.render()?;
            
            std::thread::sleep(Duration::from_millis(16)); // 60 FPS
        }
        
        Ok(())
    }
    
    fn handle_events(&mut self) -> GameResult<()> {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Close => self.running = false,
                Event::Key(key) => self.input_manager.handle_key(key),
                Event::Mouse(mouse) => self.input_manager.handle_mouse(mouse),
                Event::Resize(w, h) => self.renderer.resize(w, h),
            }
        }
        Ok(())
    }
    
    fn update(&mut self, delta_time: f32) -> GameResult<()> {
        let input_state = self.input_manager.get_state();
        
        // ワールド更新
        {
            let mut world = self.world.write().unwrap();
            world.update(delta_time, &input_state)?;
        }
        
        // UI更新
        self.ui_manager.update(&input_state)?;
        
        // ネットワーク更新
        if let Some(ref mut network) = self.network_manager {
            network.update()?;
        }
        
        Ok(())
    }
    
    fn render(&mut self) -> GameResult<()> {
        let world = self.world.read().unwrap();
        
        self.renderer.begin_frame()?;
        self.renderer.render_world(&*world)?;
        self.renderer.render_ui(&self.ui_manager)?;
        self.renderer.end_frame()?;
        
        Ok(())
    }
}
```

### ワールドシステム
```rust
pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    entities: Vec<Entity>,
    players: Vec<Player>,
    chunk_provider: Box<dyn ChunkProvider>,
    spawn_point: Vec3<i32>,
    time: u64,
    seed: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entities: Vec::new(),
            players: Vec::new(),
            chunk_provider: Box::new(DefaultChunkProvider::new()),
            spawn_point: Vec3::new(0, 64, 0),
            time: 0,
            seed: rand::thread_rng().gen(),
        }
    }
    
    pub fn get_chunk(&mut self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }
    
    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }
    
    pub fn load_chunk(&mut self, pos: ChunkPos) -> GameResult<()> {
        if !self.chunks.contains_key(&pos) {
            let chunk = self.chunk_provider.generate_chunk(pos, self.seed)?;
            self.chunks.insert(pos, chunk);
        }
        Ok(())
    }
    
    pub fn get_block(&self, pos: BlockPos) -> Option<BlockType> {
        let chunk_pos = pos.to_chunk_pos();
        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            chunk.get_block(pos.to_local_pos())
        } else {
            None
        }
    }
    
    pub fn set_block(&mut self, pos: BlockPos, block: BlockType) -> GameResult<()> {
        let chunk_pos = pos.to_chunk_pos();
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(pos.to_local_pos(), block);
            Ok(())
        } else {
            Err(GameError::WorldError(WorldError::ChunkNotFound(chunk_pos)))
        }
    }
    
    pub fn update(&mut self, delta_time: f32, input: &InputState) -> GameResult<()> {
        self.time += 1;
        
        // プレイヤー更新
        for player in &mut self.players {
            player.update(delta_time, input, self)?;
        }
        
        // エンティティ更新
        for entity in &mut self.entities {
            entity.update(delta_time, self)?;
        }
        
        // チャンク管理
        self.update_chunks()?;
        
        Ok(())
    }
    
    fn update_chunks(&mut self) -> GameResult<()> {
        // プレイヤー周辺のチャンクをロード
        for player in &self.players {
            let player_chunk = player.get_position().to_chunk_pos();
            let radius = 8; // 読み込み範囲
            
            for x in (player_chunk.x - radius)..=(player_chunk.x + radius) {
                for z in (player_chunk.z - radius)..=(player_chunk.z + radius) {
                    let chunk_pos = ChunkPos::new(x, z);
                    self.load_chunk(chunk_pos)?;
                }
            }
        }
        
        // 遠いチャンクをアンロード
        self.chunks.retain(|&pos, _| {
            // プレイヤーからの距離チェック
            for player in &self.players {
                let player_chunk = player.get_position().to_chunk_pos();
                let distance = ((pos.x - player_chunk.x).abs() + 
                               (pos.z - player_chunk.z).abs()) as f32;
                if distance <= 16.0 { // アンロード距離
                    return true;
                }
            }
            false
        });
        
        Ok(())
    }
}
```

### チャンクシステム
```rust
pub struct Chunk {
    position: ChunkPos,
    blocks: Box<[u8; 16 * 16 * 128]>,
    height_map: [u8; 16 * 16],
    entities: Vec<Entity>,
    tile_entities: HashMap<BlockPos, TileEntity>,
    is_modified: bool,
    last_accessed: Instant,
}

impl Chunk {
    pub fn new(position: ChunkPos) -> Self {
        Self {
            position,
            blocks: Box::new([0; 16 * 16 * 128]),
            height_map: [0; 16 * 16],
            entities: Vec::new(),
            tile_entities: HashMap::new(),
            is_modified: false,
            last_accessed: Instant::now(),
        }
    }
    
    pub fn get_block(&self, pos: LocalBlockPos) -> Option<BlockType> {
        if pos.x < 16 && pos.y < 128 && pos.z < 16 {
            let index = (pos.x + pos.z * 16 + pos.y * 256) as usize;
            Some(BlockType::from_id(self.blocks[index]))
        } else {
            None
        }
    }
    
    pub fn set_block(&mut self, pos: LocalBlockPos, block: BlockType) -> bool {
        if pos.x < 16 && pos.y < 128 && pos.z < 16 {
            let index = (pos.x + pos.z * 16 + pos.y * 256) as usize;
            self.blocks[index] = block.get_id();
            self.is_modified = true;
            true
        } else {
            false
        }
    }
    
    pub fn generate_terrain(&mut self, generator: &dyn TerrainGenerator) {
        for x in 0..16 {
            for z in 0..16 {
                let mut height = 0;
                for y in 0..128 {
                    let block_type = generator.generate_block(
                        self.position.x * 16 + x,
                        y,
                        self.position.z * 16 + z
                    );
                    
                    if block_type != BlockType::Air {
                        self.set_block(
                            LocalBlockPos::new(x, y, z),
                            block_type
                        );
                        height = y;
                    }
                }
                
                // 高度マップ更新
                self.height_map[x + z * 16] = height as u8;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LocalBlockPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl LocalBlockPos {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
```

### エンティティシステム
```rust
pub trait Entity {
    fn get_id(&self) -> u32;
    fn get_position(&self) -> Vec3<f64>;
    fn set_position(&mut self, pos: Vec3<f64>);
    fn get_velocity(&self) -> Vec3<f64>;
    fn set_velocity(&mut self, vel: Vec3<f64>);
    fn update(&mut self, delta_time: f32, world: &World) -> GameResult<()>;
    fn render(&self, renderer: &mut Renderer);
    fn is_alive(&self) -> bool;
}

pub struct BaseEntity {
    id: u32,
    position: Vec3<f64>,
    velocity: Vec3<f64>,
    rotation: Vec2<f32>,
    on_ground: bool,
    health: i32,
}

impl Entity for BaseEntity {
    fn get_id(&self) -> u32 { self.id }
    fn get_position(&self) -> Vec3<f64> { self.position }
    fn set_position(&mut self, pos: Vec3<f64>) { self.position = pos }
    fn get_velocity(&self) -> Vec3<f64> { self.velocity }
    fn set_velocity(&mut self, vel: Vec3<f64>) { self.velocity = vel }
    
    fn update(&mut self, delta_time: f32, world: &World) -> GameResult<()> {
        // 重力適用
        if !self.on_ground {
            self.velocity.y -= 9.81 * delta_time;
        }
        
        // 位置更新
        self.position += self.velocity * delta_time as f64;
        
        // 衝突判定
        self.handle_collisions(world)?;
        
        Ok(())
    }
    
    fn render(&self, renderer: &mut Renderer) {
        // 基本エンティティ描画
    }
    
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

pub struct Player {
    base: BaseEntity,
    username: String,
    inventory: PlayerInventory,
    game_mode: GameMode,
}

impl Entity for Player {
    fn get_id(&self) -> u32 { self.base.get_id() }
    fn get_position(&self) -> Vec3<f64> { self.base.get_position() }
    fn set_position(&mut self, pos: Vec3<f64>) { self.base.set_position(pos) }
    fn get_velocity(&self) -> Vec3<f64> { self.base.get_velocity() }
    fn set_velocity(&mut self, vel: Vec3<f64>) { self.base.set_velocity(vel) }
    
    fn update(&mut self, delta_time: f32, world: &World) -> GameResult<()> {
        // プレイヤー入力処理
        self.handle_input(world)?;
        
        // 基本エンティティ更新
        self.base.update(delta_time, world)?;
        
        Ok(())
    }
    
    fn render(&self, renderer: &mut Renderer) {
        // プレイヤー描画
    }
    
    fn is_alive(&self) -> bool {
        self.base.is_alive()
    }
}

impl Player {
    pub fn new(username: String) -> Self {
        Self {
            base: BaseEntity::new(),
            username,
            inventory: PlayerInventory::new(),
            game_mode: GameMode::Survival,
        }
    }
    
    fn handle_input(&mut self, world: &World) -> GameResult<()> {
        // キーボード・マウス入力処理
        // 移動、ジャンプ、ブロック設置など
        Ok(())
    }
}
```

## 非同期処理

### チャンクローディング
```rust
use tokio::sync::{mpsc, RwLock};
use tokio::task;

pub struct AsyncWorld {
    chunks: Arc<RwLock<HashMap<ChunkPos, Chunk>>>,
    chunk_loader: mpsc::Sender<ChunkLoadRequest>,
    chunk_receiver: Arc<Mutex<mpsc::Receiver<ChunkLoadResult>>>,
}

#[derive(Debug)]
struct ChunkLoadRequest {
    position: ChunkPos,
    sender: oneshot::Sender<Chunk>,
}

#[derive(Debug)]
struct ChunkLoadResult {
    position: ChunkPos,
    chunk: Chunk,
}

impl AsyncWorld {
    pub fn new() -> Self {
        let (load_sender, mut load_receiver) = mpsc::channel(100);
        let (result_sender, result_receiver) = mpsc::channel(100);
        
        // チャンクローダースレッド
        task::spawn(async move {
            while let Some(request) = load_receiver.recv().await {
                let chunk = generate_chunk(request.position);
                let _ = request.sender.send(chunk);
            }
        });
        
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_loader: load_sender,
            chunk_receiver: Arc::new(Mutex::new(result_receiver)),
        }
    }
    
    pub async fn load_chunk(&self, pos: ChunkPos) -> GameResult<Chunk> {
        // 既存チェック
        {
            let chunks = self.chunks.read().await;
            if let Some(chunk) = chunks.get(&pos) {
                return Ok(chunk.clone());
            }
        }
        
        // 非同期ロード要求
        let (sender, receiver) = oneshot::channel();
        self.chunk_loader.send(ChunkLoadRequest {
            position: pos,
            sender,
        }).await?;
        
        // 結果待機
        let chunk = receiver.await?;
        
        // チャンク保存
        {
            let mut chunks = self.chunks.write().await;
            chunks.insert(pos, chunk.clone());
        }
        
        Ok(chunk)
    }
}
```

## パフォーマンス最適化

### メモリプール
```rust
pub struct VecPool<T> {
    pool: Vec<Vec<T>>,
    max_size: usize,
}

impl<T> VecPool<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::new(),
            max_size,
        }
    }
    
    pub fn get(&mut self) -> Vec<T> {
        self.pool.pop().unwrap_or_else(|| Vec::new())
    }
    
    pub fn return_vec(&mut self, mut vec: Vec<T>) {
        vec.clear();
        if self.pool.len() < self.max_size {
            self.pool.push(vec);
        }
    }
}

// 使用例
static VERTEX_POOL: Lazy<Mutex<VecPool<Vertex>>> = 
    Lazy::new(|| Mutex::new(VecPool::new(100)));

pub fn render_chunk(chunk: &Chunk) -> Vec<Vertex> {
    let mut pool = VERTEX_POOL.lock().unwrap();
    let mut vertices = pool.get();
    
    // 頂点生成
    generate_vertices(chunk, &mut vertices);
    
    let result = vertices.clone();
    pool.return_vec(vertices);
    
    result
}
```

### キャッシュ戦略
```rust
use lru::LruCache;

pub struct TextureCache {
    cache: Arc<Mutex<LruCache<String, Texture>>>,
    max_size: usize,
}

impl TextureCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(max_size))),
            max_size,
        }
    }
    
    pub fn get_or_load<F>(&self, path: &str, loader: F) -> GameResult<Texture>
    where
        F: FnOnce(&str) -> GameResult<Texture>,
    {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(texture) = cache.get(path) {
            return Ok(texture.clone());
        }
        
        let texture = loader(path)?;
        cache.put(path.to_string(), texture.clone());
        Ok(texture)
    }
}
```

## テスト戦略

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_block_operations() {
        let mut chunk = Chunk::new(ChunkPos::new(0, 0));
        
        // ブロック設置
        let pos = LocalBlockPos::new(8, 64, 8);
        assert!(chunk.set_block(pos, BlockType::Stone));
        
        // ブロック取得
        assert_eq!(chunk.get_block(pos), Some(BlockType::Stone));
        
        // 範囲外テスト
        let out_of_bounds = LocalBlockPos::new(16, 64, 8);
        assert_eq!(chunk.get_block(out_of_bounds), None);
    }
    
    #[test]
    fn test_world_chunk_loading() {
        let mut world = World::new();
        let chunk_pos = ChunkPos::new(0, 0);
        
        // チャンク読み込み
        assert!(world.load_chunk(chunk_pos).is_ok());
        
        // チャンク存在確認
        assert!(world.get_chunk(chunk_pos).is_some());
    }
}
```

### 統合テスト
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_game_loop() {
        let mut game = Game::new().unwrap();
        
        // 短時間実行テスト
        let start = Instant::now();
        while start.elapsed().as_secs() < 1 {
            game.handle_events().unwrap();
            game.update(0.016).unwrap();
            game.render().unwrap();
        }
    }
}
```

## まとめ

Rust移植の主要な設計パターン：

1. **所有権ベース**: Arc/RwLock/Mutexによるスレッドセーフな共有
2. **トレイト抽象化**: 共振動作の定義と動的ディスパッチ
3. **エラーハンドリング**: Result型とカスタムエラー型
4. **非同期処理**: tokioによる並列チャンクローディング
5. **パフォーマンス最適化**: メモリプール、LRUキャッシュ
6. **テスト戦略**: 単体テストと統合テストの分離

これらのパターンを適用することで、安全で高性能なMinecraftクローンの実現が可能になります。
