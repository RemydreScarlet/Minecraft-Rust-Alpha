# 数学ユーティリティクラス分析

## Vec3（a）クラス

### クラス概要
**ファイル**: `a.java`  
**役割**: 3D整数座標、ハッシュ計算、距離計算

### フィールド変数
```java
public final int a;  // X座標
public final int b;  // Y座標  
public final int c;  // Z座標
public final int d;  // ハッシュ値（ビットパック）
int e = -1;        // 状態フラグ
float f, g, h;     // 浮動小数点座標
a i;               // 関連オブジェクト
boolean j = false;  // 使用フラグ
```

### 座標エンコーディング
```java
// コンストラクタでのハッシュ計算
this.d = var1 | var2 << 10 | var3 << 20;

// ビット構成:
// bits 0-9:   X座標 (10ビット = -512 to 511)
// bits 10-19:  Y座標 (10ビット = -512 to 511)  
// bits 20-29:  Z座標 (10ビット = -512 to 511)
// bits 30-31:  未使用
```

### 主要メソッド
```java
// 距離計算
public float a(a var1) {
    float dx = (float)(var1.a - this.a);
    float dy = (float)(var1.b - this.b);
    float dz = (float)(var1.c - this.c);
    return eo.c(dx * dx + dy * dy + dz * dz);
}

// ハッシュ比較
public boolean equals(Object var1) {
    return ((a)var1).d == this.d;
}

// ハッシュ値
public int hashCode() {
    return this.d;
}
```

## MathHelper（eo）クラス

### クラス概要
**ファイル**: `eo.java`  
**役割**: 数学関数の高速化、三角関数テーブル、座標変換

### 静的フィールド
```java
private static float[] a = new float[65536];  // sinテーブル
```

### 三角関数テーブル
```java
// 初期化
static {
    for(int i = 0; i < 65536; i++) {
        a[i] = (float)Math.sin(i * 2π / 65536.0);
    }
}

// 高速sin関数
public static final float a(float var0) {
    return a[(int)(var0 * 10430.378F) & 65535];
}

// 高速cos関数（sin + π/2）
public static final float b(float var0) {
    return a[(int)(var0 * 10430.378F + 16384.0F) & 65535];
}
```

### 座標変換関数
```java
// 床関数
public static int d(float var0) {
    int var1 = (int)var0;
    return var0 < (float)var1 ? var1 - 1 : var1;
}

public static int b(double var0) {
    int var2 = (int)var0;
    return var0 < (double)var2 ? var2 - 1 : var2;
}

// 絶対値
public static float e(float var0) {
    return var0 >= 0.0F ? var0 : -var0;
}

// 最大値
public static double a(double var0, double var2) {
    return var0 > var2 ? var0 : var2;
}

// 整数除算（床除算）
public static int a(int var0, int var1) {
    return var0 < 0 ? -((-var0 - 1) / var1) - 1 : var0 / var1;
}
```

### 高速平方根
```java
// float版
public static final float c(float var0) {
    return (float)Math.sqrt((double)var0);
}

// double版
public static final float a(double var0) {
    return (float)Math.sqrt(var0);
}
```

## 使用例

### 座標ハッシュ
```java
// チャンク座標計算
int chunkX = a.a(worldX, worldZ) >> 4;
int chunkZ = a.b(worldX, worldZ) >> 4;

// ブロック座標
int blockX = worldX & 15;
int blockZ = worldZ & 15;
```

### 距離計算
```java
// 2点間の距離
Vec3 pos1 = new Vec3(x1, y1, z1);
Vec3 pos2 = new Vec3(x2, y2, z2);
float distance = pos1.distanceTo(pos2);
```

### 角度計算
```java
// プレイヤーの向き
float yaw = MathHelper.sin(playerYaw * π / 180.0);
float pitch = MathHelper.cos(playerPitch * π / 180.0);

// 方向ベクトル
float dx = MathHelper.a(yaw);
float dz = MathHelper.b(yaw);
```

## パフォーマンス最適化

### ビット演算
- **ハッシュ計算**: ビットシフトによる高速座標エンコード
- **マスク演算**: &演算子による高速剰余計算
- **符号処理**: 条件分岐の最小化

### テーブル参照
- **三角関数**: 65536要素のルックアップテーブル
- **補間なし**: 量子化誤差を許容して速度優先
- **メモリ使用**: 256KBのテーブルを常駐

### 浮動小数点最適化
- **キャスト回避**: 直接のfloat演算
- **特殊関数**: sqrtの高速化
- **定数折りたたみ**: コンパイル時最適化

## Rust移植時の設計提案

### Vec3構造体
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    
    pub fn distance_to(&self, other: &Vec3i) -> f32 {
        let dx = (other.x - self.x) as f32;
        let dy = (other.y - self.y) as f32;
        let dz = (other.z - self.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    pub fn chunk_pos(&self) -> Vec2i {
        Vec2i::new(self.x >> 4, self.z >> 4)
    }
    
    pub fn block_pos(&self) -> Vec2i {
        Vec2i::new(self.x & 15, self.z & 15)
    }
    
    pub fn packed(&self) -> u32 {
        ((self.x as u32) & 0x3FF) |
        (((self.y as u32) & 0x3FF) << 10) |
        (((self.z as u32) & 0x3FF) << 20)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub z: i32,
}

impl Vec2i {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}
```

### MathHelper
```rust
pub struct MathHelper;

impl MathHelper {
    const SIN_TABLE_SIZE: usize = 65536;
    const SIN_SCALE: f32 = 10430.378;
    const COS_OFFSET: f32 = 16384.0;
    
    // staticでsinテーブルを保持
    static SIN_TABLE: [f32; Self::SIN_TABLE_SIZE] = Self::init_sin_table();
    
    const fn init_sin_table() -> [f32; Self::SIN_TABLE_SIZE] {
        let mut table = [0.0; Self::SIN_TABLE_SIZE];
        let mut i = 0;
        
        while i < Self::SIN_TABLE_SIZE {
            table[i] = (i as f32 * 2.0 * std::f32::consts::PI / Self::SIN_TABLE_SIZE as f32).sin();
            i += 1;
        }
        
        table
    }
    
    #[inline]
    pub fn sin(angle: f32) -> f32 {
        let index = (angle * Self::SIN_SCALE) as i32;
        Self::SIN_TABLE[(index as usize) & (Self::SIN_TABLE_SIZE - 1)]
    }
    
    #[inline]
    pub fn cos(angle: f32) -> f32 {
        let index = (angle * Self::SIN_SCALE + Self::COS_OFFSET) as i32;
        Self::SIN_TABLE[(index as usize) & (Self::SIN_TABLE_SIZE - 1)]
    }
    
    #[inline]
    pub fn floor(value: f32) -> i32 {
        let result = value as i32;
        if value < result as f32 {
            result - 1
        } else {
            result
        }
    }
    
    #[inline]
    pub fn floor_double(value: f64) -> i32 {
        let result = value as i32;
        if value < result as f64 {
            result - 1
        } else {
            result
        }
    }
    
    #[inline]
    pub fn abs(value: f32) -> f32 {
        value.abs()
    }
    
    #[inline]
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }
    
    #[inline]
    pub fn floor_div(a: i32, b: i32) -> i32 {
        if a < 0 {
            -((-a - 1) / b) - 1
        } else {
            a / b
        }
    }
    
    #[inline]
    pub fn sqrt(value: f32) -> f32 {
        value.sqrt()
    }
    
    #[inline]
    pub fn sqrt_double(value: f64) -> f32 {
        value.sqrt() as f32
    }
}
```

### 最適化戦略
1. **インライン化**: 小さな関数を全てインライン
2. **定数畳み込み**: コンパイル時定数評価
3. **SIMD**: ベクトル演算の並列化
4. **キャッシュ**: 頻繁使用値のキャッシュ

### メモリレイアウト
```rust
// パック座標によるメモリ効率化
#[derive(Clone, Copy)]
pub struct PackedPos {
    value: u32,
}

impl PackedPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            value: ((x as u32) & 0x3FF) |
                   (((y as u32) & 0x3FF) << 10) |
                   (((z as u32) & 0x3FF) << 20)
        }
    }
    
    pub fn x(&self) -> i32 {
        ((self.value & 0x3FF) as i32) - 512
    }
    
    pub fn y(&self) -> i32 {
        (((self.value >> 10) & 0x3FF) as i32) - 512
    }
    
    pub fn z(&self) -> i32 {
        (((self.value >> 20) & 0x3FF) as i32) - 512
    }
}
```

これらの数学ユーティリティはMinecraftの座標計算、物理演算、レンダリングの基礎となり、高速化が重要です。
