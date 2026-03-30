# World（cn）クラス分析

## クラス概要

**ファイル**: `cn.java`  
**役割**: ワールド全体の管理、チャンクシステム、エンティティ管理、セーブ/ロード

## 主要なフィールド変数

### ワールドデータ
- `List a`: ワールド内の全エンティティリスト
- `List b`: 読み込み済みチャンクリスト  
- `long c`: ワールド時間
- `boolean d`: 雪が積もっているか
- `Random n`: ワールド乱数生成器
- `int o, p, q`: スポーン座標 (X, Y, Z)
- `long u`: ワールドシード値
- `String w`: ワールド名

### チャンク管理
- `aw H`: チャンクプロバイダ (ChunkProvider)
- `TreeSet B`: 読み込み待ちチャンクリスト
- `Set C`: チャンク更新セット

### エンティティ管理
- `List k`: プレイヤーリスト
- `List s`: スケジュール済みタスクリスト
- `Object m`: ワールドデータストレージ

### その他
- `File t`: セーブファイルディレクトリ
- `boolean y`: マルチプレイモードか
- `boolean r`: 新規ワールドか

## 主要なメソッド

### 初期化
- `cn(File, String, long)`: ワールドコンストラクタ
  - セーブディレクトリ作成
  - セッションロック作成
  - level.dat読み込み
  - スポーン地点探索

### ワールド操作
- `a(int, int, int)`: 指定座標のブロックID取得
- `a(int, int, int, int, int, int)`: ブロック設置
- `d(int, int, int)`: チャンク存在確認
- `a(int, int)`: チャンク取得

### エンティティ管理
- `a(dm)`: エンティティをワールドに追加
- `a(kh)`: プレイヤーをワールドに追加
- `b(int, int, int, List)`: 指定範囲のエンティティ取得

### セーブ/ロード
- `a(boolean, nu)`: ワールドセーブ
- `m()`: level.dat書き込み
- `a(File, String)`: ワールドデータ読み込み

## チャンクシステム

### チャンク（gaクラス）
- **サイズ**: 16x16x128ブロック
- **データ構造**: 
  - `byte[] b`: ブロックデータ (16x16x128 = 32768バイト)
  - `mu e, f, g`: 高度マップ (照明計算用)
  - `Map l`: タイルエンティティ
  - `List[] m`: Yセクションごとのエンティティリスト

### チャンク座標系
```
ワールド座標 → チャンク座標
chunkX = worldX >> 4    (16で割る)
chunkZ = worldZ >> 4

チャンク内座標
localX = worldX & 15   (16の余り)
localZ = worldZ & 15
```

### チャンク読み込みプロセス
1. **要求**: プレイヤー位置に基づきチャンク要求
2. **生成**: チャンクプロバイダが地形生成
3. **読み込み**: ディスクから既存チャンク読み込み
4. **キャッシュ**: メモリに保持
5. **破棄**: プレイヤーから遠いチャンクを破棄

## ブロック操作

### 座標変換
```java
// ワールド座標からチャンク内インデックス
int index = (localX << 11) | (localZ << 7) | localY;

// チャンク内インデックスから座標
int localX = (index >> 11) & 15;
int localY = index & 127;
int localZ = (index >> 7) & 15;
```

### ブロック設置処理
1. チャンク取得
2. チャンク内にブロック設置
3. 隣接ブロック更新
4. 照明計算
5. レンダリング更新

## エンティティシステム

### エンティティ管理
- **追加**: `a(dm var1)` - ワールドにエンティティ追加
- **削除**: 自動的にチャンクから削除
- **更新**: 毎フレームの位置・状態更新
- **衝突**: チャンク内でのみ衝突判定

### プレイヤー管理
- **スポーン**: 安全な地面を探索してスポーン
- **リスポーン**: 最後のベッドまたはスポーン地点
- **セーブ**: プレイヤーデータをlevel.datに保存

## ワールド生成

### 地形生成アルゴリズム
1. **基本地形**: パーリンノイズで高低差生成
2. **洞穴**: ランダムウォークで洞穴掘削
3. **鉱石**: 地下に鉱石脈配置
4. **植生**: 草・木・花を地表に配置
5. **構造物**: 村やダンジョン生成

### バイオーム生成
- 温度・湿度に基づきバイオーム決定
- 各バイオームで固有のブロック配置
- 境界でスムーズな遷移

## セーブシステム

### level.dat構造
```
level.dat (NBT形式)
├── Data
│   ├── RandomSeed (long): ワールドシード
│   ├── SpawnX (int): スポーンX座標
│   ├── SpawnY (int): スポーンY座標  
│   ├── SpawnZ (int): スポーンZ座標
│   ├── Time (long): ワールド時間
│   ├── SizeOnDisk (long): ディスクサイズ
│   ├── SnowCovered (byte): 雪フラグ
│   ├── Player (compound): プレイヤーデータ
│   └── LastPlayed (long): 最終プレイ時間
└── (その他メタデータ)
```

### セーブプロセス
1. **チャンクセーブ**: 全チャンクを個別ファイルにセーブ
2. **プレイヤーセーブ**: プレイヤーデータをlevel.datに保存
3. **ワールドメタセーブ**: 時間・設定などを保存
4. **アトミック操作**: 一時ファイル→本ファイルにリネーム

## Rust移植時の設計提案

### 構造体設計
```rust
pub struct World {
    // 基本情報
    name: String,
    seed: u64,
    time: u64,
    spawn_pos: Vec3<i32>,
    
    // チャンク管理
    chunks: HashMap<ChunkPos, Chunk>,
    chunk_provider: Box<dyn ChunkProvider>,
    loaded_chunks: HashSet<ChunkPos>,
    
    // エンティティ管理
    entities: Vec<Entity>,
    players: Vec<Player>,
    scheduled_tasks: Vec<ScheduledTask>,
    
    // ファイルシステム
    save_dir: PathBuf,
    world_data: WorldData,
}

pub struct Chunk {
    // 位置情報
    pos: ChunkPos,
    
    // ブロックデータ
    blocks: Box<[u8; 16 * 16 * 128]>,
    height_map: [i8; 16 * 16],
    
    // エンティティ
    entities: Vec<Entity>,
    tile_entities: HashMap<BlockPos, TileEntity>,
    
    // 状態
    is_modified: bool,
    last_accessed: Instant,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    x: i32,
    z: i32,
}
```

### 主要トレイト
```rust
pub trait WorldManager {
    fn get_block(&self, pos: BlockPos) -> Option<u8>;
    fn set_block(&mut self, pos: BlockPos, block: u8) -> bool;
    fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk>;
    fn load_chunk(&mut self, pos: ChunkPos) -> Result<(), WorldError>;
    fn save_chunk(&self, chunk: &Chunk) -> Result<(), WorldError>;
}

pub trait ChunkProvider {
    fn generate_chunk(&mut self, pos: ChunkPos, seed: u64) -> Chunk;
    fn populate_chunk(&mut self, chunk: &mut Chunk);
}
```

### パフォーマンス考慮
1. **チャンクキャッシュ**: LRUキャッシュでメモリ効率化
2. **非同期ロード**: 別スレッドでチャンク読み込み
3. **レベルオブデティング**: 不要なチャンクから破棄
4. **メモリプール**: チャンクデータの再利用

### エラーハンドリング
```rust
#[derive(Debug)]
pub enum WorldError {
    ChunkNotFound(ChunkPos),
    InvalidPosition(BlockPos),
    SaveError(io::Error),
    LoadError(io::Error),
    CorruptedData,
}
```

このクラスはMinecraftの最大のデータ構造であり、効率的なチャンク管理と適切なセーブ/ロード処理が重要です。
