# エンティティシステム分析

## Entity（dm）クラス

### クラス概要
**ファイル**: `dm.java`  
**役割**: エンティティ基底クラス、物理演算、衝突判定

### 主要なフィールド変数
```java
public eu b = new eu(this);  // インベントリ
public byte c = 0;          // エンティティ状態
public int d = 0;           // エンティティID
public float e, f;          // 速度ベクトル
public boolean g = false;   // 地面接触フラグ
public int h = 0;           // 体力
public String i;            // エンティティ名
```

### 物理演算
```java
// 位置更新
public void j() {
    this.b.b();           // インベントリ更新
    this.e = this.f;       // 速度リセット
    
    // 移動計算
    float speed = Math.sqrt(this.an * this.an + this.ap * this.ap);
    float pitch = (float)Math.atan(-this.ao * 0.2) * 15.0f;
    
    // 地面摩擦
    this.f += (speed - this.f) * 0.4f;
    this.M += (pitch - this.M) * 0.8f;
    
    // 衝突判定
    if(this.E > 0) {
        List entities = this.ag.b(this, this.au.b(1.0, 0.0, 1.0));
        for(Entity entity : entities) {
            this.collideWith(entity);
        }
    }
}
```

### 衝突処理
```java
// エンティティ間衝突
private void h(Entity var1) {
    var1.collide(this);
}

// 地面判定
protected void b_() {
    if(this.g) {
        ++this.h;
        if(this.h == 8) {
            this.h = 0;
            this.g = false;
        }
    } else {
        this.h = 0;
    }
    this.D = (float)this.h / 8.0f;
}
```

## Player（bi）クラス

### クラス概要
**ファイル**: `bi.java`  
**役割**: プレイヤーエンティティ、入力処理、インベントリ管理

### 主要なフィールド変数
```java
public lv a;           // プレイヤーインベントリ
private Minecraft bg;  // Minecraftインスタンス
```

### プレイヤー固有機能
```java
// スキン読み込み
public bi(Minecraft var1, World var2, Session var3) {
    super(var2);
    this.bg = var1;
    if(var3 != null && var3.b != null && var3.b.length() > 0) {
        this.aY = "http://www.minecraft.net/skin/" + var3.b + ".png";
        System.out.println("Loading texture " + this.aY);
    }
    this.i = var3.b;  // プレイヤー名
}

// インベントリ操作
public void a(Entity var1) {
    int damage = this.b.a(var1);  // 武器ダメージ計算
    if(damage > 0) {
        var1.a(this, damage);  // エンティティにダメージ
        ItemStack item = this.t();  // 現在のアイテム
        if(item != null && var1 instanceof LivingEntity) {
            item.a((LivingEntity)var1);  // アイテム効果適用
            if(item.a <= 0) {
                item.a((Entity)this);  // アイテム消費
                this.u();  // インベントリ更新
            }
        }
    }
}
```

### 入力処理
```java
// キーボード入力
public void a(int var1, boolean var2) {
    this.a.a(var1, var2);  // インベントリ入力
}

// マウス入力
public void a_(Entity var1, int var2) {
    this.bg.h.a((nq)(new cd(this.bg.e, var1, this, -0.5f)));
}
```

## Item（ev）クラス

### クラス概要
**ファイル**: `ev.java`  
**役割**: アイテムスタック、アイテムプロパティ、使用処理

### 主要なフィールド変数
```java
public int a;  // アイテム数
public int b;  // アイテムID
public int c;  // アイテム種類
public int d;  // ダメージ値
```

### アイテム操作
```java
// アイテム消費
public ev a(int var1) {
    this.a -= var1;
    return new ev(this.c, var1, this.d);
}

// アイテム複製
public ev e() {
    return new ev(this.c, this.a, this.d);
}

// ダメージ適用
public void b(int var1) {
    this.d += var1;
    if(this.d > this.d()) {
        --this.a;
        if(this.a < 0) {
            this.a = 0;
        }
        this.d = 0;
    }
}
```

### アイテム使用
```java
// エンティティに使用
public boolean a(Entity var1, World var2, int var3, int var4, int var5, int var6) {
    return this.a().a(this, var1, var2, var3, var4, var5, var6);
}

// ブロックに使用
public boolean a(Entity var1, World var2, int var3, int var4, int var5) {
    return this.a().a(this, var1, var2, var3, var4, var5);
}
```

## インベントリシステム

### Inventory（eu）クラス
```java
public class eu {
    public ev[] a;  // アイテム配列
    public int b;    // 現在選択中のスロット
    
    // アイテム取得
    public ev a() {
        return this.a[this.b];
    }
    
    // アイテム設置
    public void a(int var1, ev var2) {
        this.a[var1] = var2;
    }
}
```

### PlayerInventory（lv）クラス
```java
public class lv extends eu {
    // ホットバー (0-8)
    // メインインベントリ (9-35)
    // アーマースロット (36-39)
    
    public int c() { return 36; }  // インベントリサイズ
    public int f() { return 9; }   // ホットバーサイズ
}
```

## エンティティ管理

### エンティティ登録
```java
// エンティティタイプ登録
static {
    a(Arrow.class, "Arrow", 10);
    a(Snowball.class, "Snowball", 11);
    a(ItemEntity.class, "Item", 1);
    a(LivingEntity.class, "Mob", 48);
    a(Monster.class, "Monster", 49);
}
```

### エンティティ生成
```java
// エンティティファクトリ
public static Entity a(int var1, World var2) {
    Entity var2 = (Entity)c.get(Integer.valueOf(var1));
    if(var2 != null) {
        var2.e(var2);
    } else {
        System.out.println("Skipping Entity with id " + var1);
    }
    return var2;
}
```

## 物理演算システム

### 速度計算
```java
// 重力適用
public void applyGravity() {
    if(!this.av || this.E <= 0) {
        this.ao = 0.0;  // 垂直速度
    }
    
    // 空気抵抗
    this.f += (speed - this.f) * 0.4f;
}

// 衝突応答
public void handleCollision(Entity other) {
    // 弾性衝突
    double dx = this.ak - other.ak;
    double dz = this.am - other.am;
    double distance = Math.sqrt(dx * dx + dz * dz);
    
    if(distance < 1.0) {
        // 反発処理
        double push = (1.0 - distance) / 2.0;
        this.ak += dx * push;
        this.am += dz * push;
        other.ak -= dx * push;
        other.am -= dz * push;
    }
}
```

### 地面判定
```java
// 地面接触判定
public boolean isOnGround() {
    return this.g;
}

// 地面検出
protected void checkGroundCollision() {
    // 下方ブロックチェック
    int blockBelow = this.world.a(
        (int)this.ak, 
        (int)this.al - 1, 
        (int)this.am
    );
    
    this.g = blockBelow != 0 && 
              this.al < (int)this.al + 1;
}
```

## Rust移植時の設計提案

### エンティティトレイト
```rust
pub trait Entity {
    fn get_id(&self) -> i32;
    fn get_position(&self) -> Vec3d;
    fn set_position(&mut self, pos: Vec3d);
    fn get_velocity(&self) -> Vec3d;
    fn set_velocity(&mut self, vel: Vec3d);
    fn update(&mut self, world: &World, delta_time: f32);
    fn render(&self, renderer: &mut Renderer);
    fn is_alive(&self) -> bool;
}

pub trait LivingEntity: Entity {
    fn get_health(&self) -> i32;
    fn set_health(&mut self, health: i32);
    fn damage(&mut self, amount: i32, source: &DamageSource);
    fn heal(&mut self, amount: i32);
}

pub trait PlayerEntity: LivingEntity {
    fn get_inventory(&mut self) -> &mut PlayerInventory;
    fn get_username(&self) -> &str;
    fn handle_input(&mut self, input: &InputState);
}
```

### エンティティ構造体
```rust
pub struct BaseEntity {
    id: i32,
    position: Vec3d,
    velocity: Vec3d,
    rotation: Vec2f,
    on_ground: bool,
    health: i32,
    world: Option<Weak<World>>,
    entity_type: EntityType,
}

#[derive(Clone, Copy, Debug)]
pub enum EntityType {
    Player,
    Mob(MobType),
    Item(ItemType),
    Arrow,
    Snowball,
}

pub struct Player {
    base: BaseEntity,
    username: String,
    inventory: PlayerInventory,
    skin_texture: Option<Texture>,
    game_mode: GameMode,
}
```

### アイテムシステム
```rust
#[derive(Clone, Debug)]
pub struct ItemStack {
    item_id: i32,
    count: i32,
    damage: i32,
    nbt: Option<NbtCompound>,
}

impl ItemStack {
    pub fn new(item_id: i32, count: i32) -> Self {
        Self {
            item_id,
            count,
            damage: 0,
            nbt: None,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.count <= 0
    }
    
    pub fn split(&mut self, amount: i32) -> Option<Self> {
        if self.count < amount {
            return None;
        }
        
        self.count -= amount;
        Some(Self {
            item_id: self.item_id,
            count: amount,
            damage: self.damage,
            nbt: self.nbt.clone(),
        })
    }
    
    pub fn merge(&mut self, other: ItemStack) -> Option<ItemStack> {
        if self.item_id != other.item_id || self.damage != other.damage {
            return Some(other);
        }
        
        let max_stack = self.max_stack_size();
        let total = self.count + other.count;
        
        if total <= max_stack {
            self.count = total;
            None
        } else {
            self.count = max_stack;
            Some(ItemStack {
                item_id: self.item_id,
                count: total - max_stack,
                damage: self.damage,
                nbt: other.nbt,
            })
        }
    }
}
```

### インベントリシステム
```rust
pub struct Inventory {
    slots: Vec<Option<ItemStack>>,
    selected_slot: usize,
    size: usize,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
            selected_slot: 0,
            size,
        }
    }
    
    pub fn get_item(&self, slot: usize) -> Option<&ItemStack> {
        self.slots.get(slot).and_then(|item| item.as_ref())
    }
    
    pub fn set_item(&mut self, slot: usize, item: Option<ItemStack>) -> bool {
        if slot >= self.size {
            return false;
        }
        
        self.slots[slot] = item;
        true
    }
    
    pub fn add_item(&mut self, item: ItemStack) -> Option<ItemStack> {
        // 既存スロットにスタック
        for slot in &mut self.slots {
            if let Some(ref mut existing) = slot {
                if let Some(remaining) = existing.merge(item) {
                    return Some(remaining);
                }
            }
        }
        
        // 空きスロットに設置
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(item);
                return None;
            }
        }
        
        Some(item)  // インベントリ満杯
    }
}

pub struct PlayerInventory {
    main_inventory: Inventory,    // 27スロット
    hotbar: Inventory,           // 9スロット
    armor: Inventory,            // 4スロット
    selected_hotbar: usize,
}

impl PlayerInventory {
    pub fn new() -> Self {
        Self {
            main_inventory: Inventory::new(27),
            hotbar: Inventory::new(9),
            armor: Inventory::new(4),
            selected_hotbar: 0,
        }
    }
    
    pub fn get_selected_item(&self) -> Option<&ItemStack> {
        self.hotbar.get_item(self.selected_hotbar)
    }
    
    pub fn set_selected_hotbar(&mut self, index: usize) {
        self.selected_hotbar = index.min(8);
    }
}
```

### 物理演算
```rust
pub struct PhysicsSystem {
    gravity: f32,
    air_resistance: f32,
    ground_friction: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            gravity: -9.81,
            air_resistance: 0.02,
            ground_friction: 0.6,
        }
    }
    
    pub fn update_entity(&self, entity: &mut dyn Entity, world: &World, delta_time: f32) {
        // 重力適用
        if !entity.is_on_ground() {
            let mut velocity = entity.get_velocity();
            velocity.y += self.gravity * delta_time;
            entity.set_velocity(velocity);
        }
        
        // 空気抵抗
        let mut velocity = entity.get_velocity();
        velocity *= (1.0 - self.air_resistance).powf(delta_time);
        entity.set_velocity(velocity);
        
        // 地面摩擦
        if entity.is_on_ground() {
            let mut velocity = entity.get_velocity();
            velocity.x *= self.ground_friction;
            velocity.z *= self.ground_friction;
            entity.set_velocity(velocity);
        }
        
        // 衝突検出と応答
        self.handle_collisions(entity, world);
    }
    
    fn handle_collisions(&self, entity: &mut dyn Entity, world: &World) {
        // ブロック衝突
        self.check_block_collisions(entity, world);
        
        // エンティティ衝突
        self.check_entity_collisions(entity, world);
    }
}
```

このエンティティシステムはMinecraftの動的な要素の中核であり、効率的な物理演算と適切な状態管理が重要です。
