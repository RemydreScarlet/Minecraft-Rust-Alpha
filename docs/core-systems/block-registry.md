# Block（ly）クラス分析

## クラス概要

**ファイル**: `ly.java`  
**役割**: ブロックレジストリ、256種類のブロック定義、ブロックプロパティ管理

## ブロックレジストリ構造

### 静的フィールド
- `ly[] n`: ブロックインスタンス配列 [256]
- `boolean[] o`: 固体ブロックフラグ [256]
- `boolean[] p`: 不透明ブロックフラグ [256]  
- `boolean[] q`: レンダリングブロックフラグ [256]
- `int[] r`: 光透過率 [256]
- `boolean[] s`: 立方体ブロックフラグ [256]
- `int[] t`: 発光レベル [256]

### ブロック定数
```java
// 基本ブロック (ID 0-20)
public static final ly u = new ce(1, 1);      // 石
public static final my v = new my(2);          // 土
public static final ly w = new hz(3, 2);      // 草ブロック
public static final ly x = new ly(4, 16, gb.d); // コバーストーン
public static final ly y = new ly(5, 4, gb.c);  // 木
public static final ly z = new dt(6, 15);       // サボテン
public static final ly A = new ly(7, 17, gb.d); // ベッドロック
public static final ly B = new hv(8, gb.f);      // 流水
public static final ly C = new hn(9, gb.f);      // 停滞水
public static final ly D = new hv(10, gb.g);     // 流溶岩
public static final ly E = new hn(11, gb.g);     // 停滞溶岩
public static final ly F = new dh(12, 18);      // 砂
public static final ly G = new gz(13, 19);      // 砂利
public static final ly H = new gw(14, 32);      // 金鉱石
public static final ly I = new gw(15, 33);      // 鉄鉱石
public static final ly J = new gw(16, 34);      // 石炭鉱石
public static final ly K = new mg(17);           // 木
public static final iz L = new iz(18, 52);      // 木材
public static final ly M = new ng(19);           // スポンジ
public static final ly N = new ct(20, 49, gb.o, false); // ガラス
```

## ブロックカテゴリ分類

### 自然ブロック (0-20)
- **石 (1)**: 基本建築材、採掘で丸石
- **土 (2)**: 草の下、農業に使用
- **草ブロック (3)**: 地表、動物スポーン
- **コバーストーン (4)**: 16色の染料可能
- **木 (5)**: 4種類、建築・クラフト素材
- **サボテン (6)**: 砂漠、緑色染料
- **ベッドロック (7)**: スポーンポイント設定
- **水 (8-9)**: 流体、農業・製錬に使用
- **溶岩 (10-11)**: 流体、光源・ダメージ源
- **砂 (12)**: 砂漠、ガラス製造
- **砂利 (13)**: 地下、採掘で丸石
- **金鉱石 (14)**: 地下、精錬で金インゴット
- **鉄鉱石 (15)**: 地下、精錬で鉄インゴット  
- **石炭鉱石 (16)**: 地下、燃料として使用
- **木 (17)**: 4種類、建築・クラフト素材
- **木材 (18)**: 板、階段、柵などに加工
- **スポンジ (19)**: 水吸収、海底神殿
- **ガラス (20)**: 透明、光透過

### 鉱石ブロック (21-50)
- **ラピス鉱石 (21)**: 青色染料
- **ラピスブロック (22)**: 装飾ブロック
- **ディスペンサー (23)**: レッドストーン回路
- **サンドストーン (24)**: 砂の固まり
- **音符ブロック (25)**: 音の生成
- **ベッド (26)**: スリープ・スポーン設定
- **レール (27-28)**: トロッコ用
- **石レンガ (29-33)**: 建築材
- **巨大キノコ (34-50)**: 特殊構造物

### 装飾ブロック (51-84)
- **火 (51)**: 光源・ダメージ
- ** Mobスポナー (52)**: Mob生成
- **オークの木階段 (53)**: 階段ブロック
- **チェスト (54)**: アイテム保管
- **レッドストーンワイヤー (55)**: 信号伝達
- **鉱石ブロック (56)**: 鉱石の固まり
- **ダイヤモンド鉱石 (57)**: 最強の鉱石
- **作業台 (58)**: クラフト
- **小麦 (59)**: 農業、パン製造
- **土 (60)**: 耕地、農業
- **炉 (61)**: 鉱石精錬・調理
- **看板 (62-63)**: テキスト表示
- **ドア (64-71)**: 6種類の木・鉄ドア
- **レバー (72)**: レッドストーン入力
- **プレッシャープレート (73-74)**: 検出装置
- **レッドストーントーチ (75)**: レッドストーン光源
- **石ボタン (76)**: レッドストーン入力
- **雪 (77)**: 寒冷バイオーム
- **氷 (78)**: 滑り・水凍結
- **雪ブロック (79)**: 雪の積もり
- **サボテン (80)**: 緑色染料
- **粘土 (81)**: レンガ・テラコッタ製造
- **サトウキビ (82)**: 紙・本の材料
- **ジュークボックス (83)**: 音の生成
- **フェンス (84)**: 動物囲い

### 機能ブロック (85-106)
- **ニンジン (85)**: クラフト・ポーション
- **ジャック・オ・ランタン (86)**: 光源
- **トラップドチェスト (87)**: 罠
- **モンスタースポナー (88)**: Mob生成
- **石レンガの階段 (89-92)**: 階段
- **レッドストーンリピーター (93)**: 回路遅延
- **木材 (94-101)**: 板・階段・柵
- **ジャングルの木 (102-106)**: ジャングル固有

## ブロックプロパティ

### StepSound（bbクラス）
```java
public class bb {
    public final String a;  // サウンド名
    public final float b;   // 音量
    public final float c;   // ピッチ
}
```

**サウンド種類**:
- `stone`: 石系ブロック
- `wood`: 木系ブロック  
- `gravel`: 砂利
- `grass`: 草・土
- `sand`: 砂
- `cloth`: 布系

### ブロック特性
- **固体性**: エンティティが立てるか
- **不透明性**: 光を遮断するか
- **レンダリング**: 描画するか
- **光透過**: 光の透過率
- **発光**: 自身が光を発するか

## ブロック操作メソッド

### 基本操作
```java
// 座標変換
public cf f(cn var1, int var2, int var3, int var4) // テクスチャUV座標
public int a(int var1, int var2)                    // ブロックID取得
public boolean c(nm var1, int var2, int var3, int var4) // 衝突判定

// ブロック更新
public void a(cn var1, int var2, int var3, int var4, Random var5)
public boolean c(nm var1, int var2, int var3, int var4, int var5)
```

### 特殊ブロック処理
- **液体**: 水・溶岩の流動計算
- **レッドストーン**: 信号伝達処理
- **作物**: 成長・収穫ロジック
- **容器**: チェスト・かまどのインベントリ

## テクスチャ座標計算

### UVマッピング
```java
// terrain.png (16x16グリッド、各16x16ピクセル)
public cf f(cn var1, int var2, int var3, int var4) {
    return cf.b(
        (double)var2 + this.bf,  // U開始
        (double)var3 + this.bg,  // V開始  
        (double)var4 + this.bh,  // U終了
        (double)var2 + this.bi,  // V終了
        (double)var3 + this.bj,  // アニメーション
        (double)var4 + this.bk   // アニメーション
    );
}
```

### テクスチャアニメーション
- **水**: 液体アニメーション
- **溶岩**: 液体アニメーション  
- **火**: 炎の揺れアニメーション
- **レッドストーン**: 点滅アニメーション

## Rust移植時の設計提案

### ブロック列挙
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 3,
    Dirt = 2,
    Cobblestone = 4,
    Wood = 5,
    // ... 256種類
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    id: BlockType,
    metadata: u8,
}

impl Block {
    pub fn new(id: BlockType, metadata: u8) -> Self {
        Self { id, metadata }
    }
    
    pub fn is_solid(&self) -> bool {
        BLOCK_PROPERTIES[self.id as usize].is_solid
    }
    
    pub fn is_opaque(&self) -> bool {
        BLOCK_PROPERTIES[self.id as usize].is_opaque
    }
    
    pub fn get_light_opacity(&self) -> u8 {
        BLOCK_PROPERTIES[self.id as usize].light_opacity
    }
}
```

### ブロックプロパティ
```rust
#[derive(Clone, Copy)]
pub struct BlockProperties {
    is_solid: bool,
    is_opaque: bool,
    is_renderable: bool,
    light_opacity: u8,
    emitted_light: u8,
    step_sound: StepSound,
    hardness: f32,
    resistance: f32,
}

const BLOCK_PROPERTIES: [BlockProperties; 256] = {
    // 初期化リスト
};
```

### ブロックレジストリ
```rust
pub struct BlockRegistry {
    blocks: [Option<Box<dyn BlockBehavior>>; 256],
    properties: [BlockProperties; 256],
}

impl BlockRegistry {
    pub fn get_block(&self, id: u8) -> Option<&dyn BlockBehavior> {
        self.blocks[id as usize].as_deref()
    }
    
    pub fn register_block(&mut self, id: u8, block: Box<dyn BlockBehavior>) {
        self.blocks[id as usize] = Some(block);
    }
}
```

### トレイト設計
```rust
pub trait BlockBehavior {
    fn get_id(&self) -> u8;
    fn get_properties(&self) -> BlockProperties;
    fn on_place(&self, world: &mut World, pos: BlockPos);
    fn on_break(&self, world: &mut World, pos: BlockPos);
    fn on_random_tick(&self, world: &mut World, pos: BlockPos, rand: &mut Random);
    fn get_texture_coords(&self, face: Direction, metadata: u8) -> TextureCoords;
}

// 特殊ブロック用トレイト
pub trait LiquidBlock: BlockBehavior {
    fn flow(&self, world: &mut World, pos: BlockPos);
    fn get_flow_decay(&self) -> u8;
}

pub trait RedstoneBlock: BlockBehavior {
    fn update_redstone(&self, world: &mut World, pos: BlockPos);
    fn can_power(&self) -> bool;
}
```

### パフォーマンス最適化
1. **ビットフィールド**: ブロックプロパティをビットで管理
2. **ルックアップテーブル**: 高速なプロパティアクセス
3. **インライン化**: 頻繁呼び出しの関数をインライン
4. **キャッシュ**: テクスチャ座標のキャッシュ

このブロックシステムはMinecraftの基本的な構成要素であり、効率的なデータアクセスと柔軟な拡張性が重要です。
