# クラス依存関係マップ

## 概要

Minecraft Alpha v1.1.2_01のクラス間の依存関係を可視化し、システム全体のアーキテクチャを理解します。

## コアシステム階層

### レイヤー1: アプリケーションレベル
```
Minecraft (net.minecraft.client.Minecraft)
├── ゲームループ管理
├── ウィンドウ管理
├── リソース管理
└── システム初期化
```

### レイヤー2: 主要サブシステム
```
Minecraft
├── cn (World) - ワールド管理
├── e (RenderEngine) - レンダリング
├── bi (Player) - プレイヤー管理
├── bh (GuiScreen) - UI管理
└── fn (Packet) - ネットワーク
```

### レイヤー3: サポートシステム
```
World (cn)
├── ga (Chunk) - チャンク管理
├── ly (Block) - ブロック管理
├── dm (Entity) - エンティティ管理
├── ev (Item) - アイテム管理
└── aw (ChunkProvider) - チャンク生成

RenderEngine (e)
├── ho (Tessellator) - 頂点処理
├── ey (TextureManager) - テクスチャ管理
├── bn (ChunkRenderer) - チャンクレンダリング
└── kx (EntityRenderer) - エンティティレンダリング

GuiScreen (bh)
├── kd (FontRenderer) - フォント描画
├── fk (Button) - ボタン管理
├── lu (HUD) - HUD描画
└── cx (MainMenu) - メインメニュー
```

## 主要な依存関係

### Minecraft → 主要システム
```java
public class Minecraft {
    cn e;           // Worldインスタンス
    e f;            // RenderEngineインスタンス
    bi g;           // Playerインスタンス
    bh p;           // 現在のGUI画面
    ey n;           // TextureManager
    bq h;           // EntityRenderer
    kd o;           // FontRenderer
    lu u;           // HUDレンダラー
}
```

### World → サブシステム
```java
public class cn implements nm {
    List a;         // 全エンティティリスト
    List b;         // 読み込み済みチャンクリスト
    aw H;           // ChunkProvider
    List k;         // プレイヤーリスト
    List s;         // スケジュール済みタスク
    File t;         // セーブディレクトリ
}
```

### RenderEngine → レンダリングコンポーネント
```java
public class e implements im {
    List a;         // エンティティレンダラーリスト
    cn k;           // 現在のワールド
    ey l;           // TextureManager
    bn[] n, o;      // チャンクレンダラー配列
    bc u;           // ブロックレンダラー
}
```

### Entity → ゲームロジック
```java
public class dm extends ge {
    eu b;           // インベントリ
    cn ag;          // ワールド参照
    cf au;          // バウンディングボックス
}

public class bi extends dm {
    lv a;           // プレイヤーインベントリ
    Minecraft bg;    // Minecraftインスタンス
}
```

## データフロー

### ゲームループ
```
Minecraft.run()
├── handleInput()          // 入力処理
│   ├── Keyboard/Mouse
│   └── GuiScreen.handleInput()
├── updateGame()          // ゲーム更新
│   ├── World.update()
│   │   ├── Entity.update()
│   │   ├── Chunk.update()
│   │   └── Block.update()
│   └── Player.update()
└── render()              // レンダリング
    ├── RenderEngine.renderWorld()
    │   ├── ChunkRenderer.render()
    │   └── EntityRenderer.render()
    ├── GuiScreen.render()
    └── HUD.render()
```

### ワールドデータフロー
```
World
├── ChunkProvider.generateChunk()
│   ├── TerrainGenerator
│   ├── StructureGenerator
│   └── BiomeGenerator
├── Chunk.load/save()
│   ├── NBTデータ処理
│   └── ファイルI/O
└── Entity/Item管理
    ├── Entity.spawn/despawn()
    └── Item.drop/pickup()
```

### レンダリングパイプライン
```
RenderEngine
├── 視錐カリング
│   ├── Chunkの可視判定
│   └── Entityの可視判定
├── チャンクレンダリング
│   ├── Tessellator.begin()
│   ├── Block.render()
│   └── Tessellator.draw()
├── エンティティレンダリング
│   ├── Model.render()
│   └── Animation.update()
└── UIレンダリング
    ├── GuiScreen.render()
    └── HUD.render()
```

## インターフェースと抽象化

### 主要インターフェース
```java
// ワールドインターフェース
interface nm {
    // ワールド操作メソッド
}

// レンダリングインターフェース  
interface im {
    // レンダリングメソッド
}

// エンティティインターフェース
interface kh {
    // エンティティ操作メソッド
}

// GUIインターフェース
interface gh {
    // GUI操作メソッド
}
```

### 抽象クラス階層
```
ge (EntityBase)
├── dm (LivingEntity)
│   └── bi (Player)
├── dx (ItemEntity)
└── その他エンティティ

fn (PacketBase)
├── eh (PositionPacket)
├── ij (ChatPacket)
└── その他パケット

bh (GuiScreenBase)
├── cx (MainMenu)
├── ay (OptionsMenu)
└── その他GUI
```

## 静的依存関係

### レジストリシステム
```java
// ブロックレジストリ
public class ly {
    public static final ly[] n = new ly[256];
    public static final boolean[] o = new boolean[256];
    // ブロック定数定義
}

// エンティティレジストリ
public class ew {
    static {
        a(Arrow.class, "Arrow", 10);
        a(ItemEntity.class, "Item", 1);
        // エンティティ登録
    }
}

// パケットレジストリ
public class fn {
    private static Map a = new HashMap();
    static {
        a(0, KeepAlive.class);
        a(1, Login.class);
        // パケット登録
    }
}
```

### シングルトンパターン
```java
// Tessellator
public class ho {
    public static final ho a = new ho(2097152);
}

// MathHelper
public class eo {
    private static float[] a = new float[65536];
}
```

## パフォーマンス最適化の依存関係

### キャッシュシステム
```
TextureManager (ey)
├── テクスチャキャッシュ
├── バインドキャッシュ
└── アトラス管理

ChunkRenderer (bn)
├── メッシュキャッシュ
├── VBOキャッシュ
└── オクルージョンキャッシュ
```

### プーリングシステム
```
Tessellator (ho)
├── 頂点バッファプール
├── インデックスバッファプール
└── コマンドバッファプール
```

## Rust移植時のアーキテクチャ提案

### モジュール構成
```rust
// src/main.rs
mod minecraft;
mod world;
mod rendering;
mod entities;
mod ui;
mod network;
mod utils;

fn main() {
    let mut game = Minecraft::new();
    game.run();
}
```

### 依存関係管理
```rust
// trait定義による疎結合
pub trait World {
    fn get_block(&self, pos: BlockPos) -> Option<Block>;
    fn set_block(&mut self, pos: BlockPos, block: Block);
    fn get_entities(&self) -> &[Entity];
}

pub trait Renderer {
    fn render_world(&mut self, world: &World, camera: &Camera);
    fn render_entities(&mut self, entities: &[Entity]);
    fn render_ui(&mut self, ui: &GuiManager);
}

// 依存注入
pub struct Game {
    world: Box<dyn World>,
    renderer: Box<dyn Renderer>,
    ui_manager: GuiManager,
}
```

### コンポーネントアーキテクチャ
```rust
// ECS (Entity Component System) アプローチ
pub struct World {
    entities: EntityStore,
    components: ComponentStore,
    systems: Vec<Box<dyn System>>,
}

pub trait System {
    fn update(&mut self, world: &mut World, delta_time: f32);
}

// システム例
pub struct PhysicsSystem;
pub struct RenderSystem;
pub struct NetworkSystem;
```

### 非同期処理
```rust
use tokio;

pub struct AsyncWorld {
    chunks: Arc<RwLock<HashMap<ChunkPos, Chunk>>>,
    chunk_loader: tokio::task::JoinHandle<()>,
    network_handler: tokio::task::JoinHandle<()>,
}

impl AsyncWorld {
    pub async fn load_chunk(&self, pos: ChunkPos) -> Result<Chunk, WorldError> {
        // 非同期チャンク読み込み
    }
    
    pub async fn handle_packets(&self) -> Result<(), NetworkError> {
        // 非同期パケット処理
    }
}
```

## まとめ

Minecraft Alphaのアーキテクチャは以下の特徴を持っています：

1. **階層構造**: 明確なレイヤー分けによる責任分離
2. **静的登録**: レジストリパターンによる動的型管理
3. **シングルトン**: リソース共有のためのグローバルインスタンス
4. **密結合**: Javaの制約による直接的な依存関係
5. **パフォーマンス重視**: キャッシュやプーリングによる最適化

Rust移植ではこれらの特徴を踏まえつつ、所有権システム、トレイト、非同期処理を活用したよりモダンなアーキテクチャを目指すことができます。
