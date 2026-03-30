# RenderEngine（e）クラス分析

## クラス概要

**ファイル**: `e.java`  
**役割**: 3Dレンダリングエンジン、チャンクレンダリング管理、オクルージョン判定

## 主要なフィールド変数

### レンダリング管理
- `List a`: エンティティレンダラーリスト
- `cn k`: 現在のワールド
- `ey l`: テクスチャマネージャ
- `bn[] n, o`: チャンクレンダラー配列（ダブルバッファ）
- `int p, q, r`: チャンクグリッドサイズ
- `bc u`: ブロックレンダラー

### オクルージョン判定
- `IntBuffer v`: オクルージョンクエリバッファ
- `boolean w`: オクルージョン有効フラグ
- `kw[] S`: オクルージョン結果（4方向）

### レンダリング状態
- `double f, g, h`: カメラ位置
- `float i`: 視野角度
- `int j`: レンダリング距離
- `List R`: レンダリング対象チャンクリスト

## 主要なメソッド

### 初期化
- `e(Minecraft, ey)`: レンダリングエンジン初期化
  - ディスプレイリスト生成
  - スカイボックスレンダリング
  - チャンクレンダラー生成

### チャンクレンダリング
- `a()`: チャンクレンダリングセットアップ
  - ビューポート計算
  - チャンクグリッド生成
  - レンダラー初期化

```java
// ビューポート計算
int viewDistance = 64 << 3; // 512
int chunksX = viewDistance / 16 + 2; // 34x34グリッド
int chunksZ = viewDistance / 16 + 2;
```

### スカイボックスレンダリング
- `f()`: スカイボックス生成
  - 1500個の星をランダム配置
  - 球面座標変換
  - 明るさ・色の計算

```java
// 星の生成アルゴリズム
for(int i = 0; i < 1500; i++) {
    double theta = random * 2π
    double phi = random * π
    
    // 球面座標変換
    double x = sin(φ) * cos(θ)
    double y = cos(φ)
    double z = sin(φ) * sin(θ)
    
    // 明るさ計算
    double brightness = random * 0.5 + 0.25
}
```

### エンティティレンダリング
- `a(aj, oe, float)`: エンティティレンダリング
  - 視野内エンティティ収集
  - 距離ソート
  - レンダリング実行

### オクルージョン判定
- `a(dm, int, double)`: エンティティのオクルージョン判定
  - ビューポートからエンティティ判定
  - 結果をバッファに保存

## Tessellator（ho）クラス

### 概要
**ファイル**: `ho.java`  
**役割**: 頂点データ管理、OpenGLプリミティブ描画

### 主要な機能
- **頂点バッファ管理**: ByteBufferによる頂点データ
- **属性設定**: 位置・色・テクスチャ座標・法線
- **描画モード**: 点・線・三角形・四角形

### 頂点フォーマット
```java
// 頂点データ構造（32バイト/頂点）
struct Vertex {
    float position[3];    // 12バイト (0-11)
    float texture[2];     // 8バイト (12-19) 
    int color;            // 4バイト (20-23)
    float normal[3];      // 12バイト (24-31)
}
```

### 描画プロセス
```java
tessellator.start();  // 開始
tessellator.addVertex(x, y, z, u, v);  // 頂点追加
tessellator.draw();    // 描画実行
```

## レンダリングパイプライン

### 1. ビューポート計算
```java
// プレイヤー位置からビューポート計算
int chunksX = (int)(playerX / 16.0) - renderDistance;
int chunksZ = (int)(playerZ / 16.0) - renderDistance;
int chunksWidth = renderDistance * 2 + 1;
```

### 2. チャンクソート
```java
// 距離順にチャンクソート
Arrays.sort(chunks, new Comparator<Chunk>() {
    public int compare(Chunk a, Chunk b) {
        return distance(a) - distance(b);
    }
});
```

### 3. チャンクレンダリング
```java
for(int x = 0; x < gridWidth; x++) {
    for(int z = 0; z < gridWidth; z++) {
        Chunk chunk = chunks[x * gridWidth + z];
        if(chunk != null && chunk.isVisible) {
            renderChunk(chunk);
        }
    }
}
```

## オクルージョンシステム

### アルゴリズム
1. **クエリ発行**: 各エンティティからオクルージョンクエリ発行
2. **結果取得**: 次フレームで結果を取得
3. **判定**: 結果に基づき描画/非描画判定

### 実装
```java
// オクルージョンクエリ発行
GL11.glBeginQueryARB(GL_SAMPLES_PASSED, queryId);
renderEntity();
GL11.glEndQueryARB(GL_SAMPLES_PASSED);

// 結果取得
int result = glGetQueryObjectiARB(queryId);
boolean isVisible = result > 0;
```

## パフォーマンス最適化

### チャンクレンダリング
- **フラスタリング**: チャンク内の面をバッチ処理
- **ビューポートカリング**: 視野外チャンクを除外
- **レベルオブデティング**: 遠距離チャンクから破棄

### 頂点バッファ
- **VBO使用**: 頂点バッファオブジェクト
- **インスタンシング**: 同じメッシュの再利用
- **メモリプール**: バッファの再利用

### テクスチャ管理
- **バインド最小化**: テクスチャ切り替え削減
- **アトラス化**: 複数テクスチャを1つに結合
- **Mipmap**: 距離によるLOD切り替え

## Rust移植時の設計提案

### レンダリングエンジン
```rust
pub struct RenderEngine {
    world: Option<Arc<RwLock<World>>>,
    texture_manager: Arc<TextureManager>,
    chunk_renderers: Vec<ChunkRenderer>,
    entity_renderers: Vec<Box<dyn EntityRenderer>>,
    
    // レンダリング状態
    camera: Camera,
    view_distance: i32,
    frustum: Frustum,
    
    // オクルージョン
    occlusion_queries: Vec<OcclusionQuery>,
    occlusion_enabled: bool,
}

impl RenderEngine {
    pub fn new(world: Arc<RwLock<World>>, textures: Arc<TextureManager>) -> Self {
        Self {
            world: Some(world),
            texture_manager: textures,
            chunk_renderers: Vec::new(),
            entity_renderers: Vec::new(),
            camera: Camera::new(),
            view_distance: 8,
            frustum: Frustum::new(),
            occlusion_queries: Vec::new(),
            occlusion_enabled: true,
        }
    }
    
    pub fn render_world(&mut self, camera: &Camera, delta_time: f32) {
        self.setup_render_state();
        self.render_skybox();
        self.render_chunks(camera);
        self.render_entities(camera);
        self.cleanup();
    }
}
```

### テッセレータ
```rust
pub struct Tessellator {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    primitive_type: PrimitiveType,
    is_drawing: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    texture_coords: [f32; 2],
    color: [u8; 4],
    normal: [f32; 3],
}

impl Tessellator {
    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        self.vertices.clear();
        self.indices.clear();
        self.primitive_type = primitive_type;
        self.is_drawing = true;
    }
    
    pub fn vertex(&mut self, x: f32, y: f32, z: f32, u: f32, v: f32) {
        if !self.is_drawing {
            return;
        }
        
        self.vertices.push(Vertex {
            position: [x, y, z],
            texture_coords: [u, v],
            color: [255, 255, 255, 255],
            normal: [0.0, 1.0, 0.0],
        });
    }
    
    pub fn end(&mut self) -> Mesh {
        self.is_drawing = false;
        Mesh::new(self.vertices.clone(), self.indices.clone())
    }
}
```

### チャンクレンダラー
```rust
pub struct ChunkRenderer {
    chunk: Arc<Chunk>,
    mesh: Option<Mesh>,
    last_update: u64,
    distance: f32,
}

impl ChunkRenderer {
    pub fn render(&mut self, camera: &Camera, frustum: &Frustum) {
        if !self.should_render(camera, frustum) {
            return;
        }
        
        if self.needs_rebuild() {
            self.rebuild_mesh();
        }
        
        if let Some(ref mesh) = self.mesh {
            mesh.draw();
        }
    }
    
    fn rebuild_mesh(&mut self) {
        let chunk = self.chunk.read().unwrap();
        self.mesh = Some(self.generate_chunk_mesh(&chunk));
    }
}
```

### オクルージョン
```rust
pub struct OcclusionQuery {
    id: gl::Query,
    result: Option<bool>,
    pending: bool,
}

impl OcclusionQuery {
    pub fn begin(&mut self) {
        unsafe {
            gl::BeginQuery(gl::SAMPLES_PASSED, self.id);
        }
        self.pending = true;
        self.result = None;
    }
    
    pub fn end(&mut self) {
        unsafe {
            gl::EndQuery(gl::SAMPLES_PASSED);
        }
    }
    
    pub fn get_result(&mut self) -> Option<bool> {
        if self.pending {
            let mut result = 0;
            unsafe {
                gl::GetQueryObjectuiv(self.id, gl::QUERY_RESULT, &mut result);
            }
            
            if result != 0 {
                self.result = Some(result > 0);
                self.pending = false;
                self.result
            } else {
                None
            }
        } else {
            self.result
        }
    }
}
```

### パフォーマンス考慮
1. **並列レンダリング**: 複数スレッドでのチャンク処理
2. **CPU-GPU同期**: 非同期コマンドバッファ
3. **メモリ管理**: アリケータによる頂点データ管理
4. **キャッシュ戦略**: LRUによるメッシュキャッシュ

このレンダリングシステムはMinecraftの視覚的表現の中核であり、効率的なバッチ処理と適切なカリングが重要です。
