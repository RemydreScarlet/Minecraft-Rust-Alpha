# Minecraft Alpha モブ（Mob）行動アルゴリズム解析

## 概要

Minecraft Alpha のモブシステムは、シンプルながら効果的な AI アルゴリズムを使用してエンティティの行動を制御します。この文書では、オリジナルの Java コードを分析し、モブの移動、戦闘、その他の行動パターンを Rust で再実装するための詳細なガイドを提供します。

## 1. 基本的なエンティティアーキテクチャ

### 1.1 エンティティの基本構造

Minecraft Alpha のすべてのエンティティは `kh` クラスを継承します：

```java
// kh.java - Entity 基底クラスの主要フィールド
public abstract class kh {
    public double ak, al, am;      // 位置座標 (x, y, z)
    public double an, ao, ap;      // 速度ベクトル
    public float aq, ar;           // 向き (yaw, pitch)
    public cf au;                  // バウンディングボックス
    public boolean av, aw, ax;     // 地面接触、水中、溶岩などの状態
    public boolean aA;             // 死亡状態
    public float aC, aD;           // 幅と高さ
    public Random aQ;              // 乱数生成器
    public int aW, aX;             // 体力、無敵時間
}
```

**Rust 実装への示唆：**
```rust
use rand::Rng;

pub struct Entity {
    // 位置と速度
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Vec2,  // (yaw, pitch)
    
    // 物理状態
    pub bounding_box: BoundingBox,
    pub on_ground: bool,
    pub in_water: bool,
    pub in_lava: bool,
    pub is_dead: bool,
    
    // 物理特性
    pub width: f32,
    pub height: f32,
    pub step_height: f32,
    
    // 状態
    pub health: i32,
    pub hurt_time: i32,
    pub death_time: i32,
    
    // 乱数生成器
    pub rng: ThreadRng,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,  // yaw
    pub y: f32,  // pitch
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}
```

### 1.2 モブ固有の拡張

生き物（`ge` クラス）はエンティティを継承し、追加の機能を提供：

```java
// ge.java - LivingEntity クラスの主要フィールド
public class ge extends kh {
    public int j = 20;              // 最大体力
    public float k, l, m;           // 移動関連のパラメータ
    public boolean t = true;        // 攻撃可能フラグ
    public String u = "/char.png";  // テクスチャパス
    public int E = 10;              // 現在の体力
    public int F;                   // 前回の体力
    public float I = 0.0F;          // 攻撃方向
    public int J = 0;               // 死亡タイマー
    public float P = 0.1F;          // スケール係数
    private kh h;                   // 攻撃対象
    private int i = 0;               // 攻撃持続時間
}
```

**Rust 実装：**
```rust
pub struct LivingEntity {
    pub entity: Entity,
    
    // 生存関連
    pub max_health: i32,
    pub current_health: i32,
    pub prev_health: i32,
    pub attack_direction: f32,
    pub death_timer: i32,
    
    // 行動関連
    pub move_speed: f32,
    pub jump_height: f32,
    pub scale: f32,
    pub can_attack: bool,
    pub texture_path: String,
    
    // AI 関連
    pub attack_target: Option<EntityId>,
    pub attack_duration: i32,
    
    // 移動状態
    pub move_forward: f32,
    pub move_strafing: f32,
    pub is_jumping: bool,
}

pub type EntityId = u64;  // エンティティの一意識別子
```

## 2. 移動アルゴリズム

### 2.1 基本的な移動処理

Minecraft Alpha の移動は重力、摩擦、地形衝突を考慮：

```java
// ge.java - 移動処理の主要部分
public void b(float var1, float var2) {
    double var3;
    if(this.g_()) {  // 水中
        var3 = this.al;
        this.a(var1, var2, 0.02F);
        this.c(this.an, this.ao, this.ap);
        this.an *= 0.800000011920929D;
        this.ao *= 0.800000011920929D;
        this.ap *= 0.800000011920929D;
        this.ao -= 0.02D;
    } else if(this.G()) {  // 溶岩中
        // 溶岩中の移動処理（水中より遅い）
        var3 = this.al;
        this.a(var1, var2, 0.02F);
        this.c(this.an, this.ao, this.ap);
        this.an *= 0.5D;
        this.ao *= 0.5D;
        this.ap *= 0.5D;
        this.ao -= 0.02D;
    } else {  // 陸上
        float var8 = 0.91F;  // 氷上でない時の摩擦係数
        if(this.av) {  // 氷上
            var8 = 0.54600006F;
        }
        
        float var9 = 0.16277136F / (var8 * var8 * var8);
        this.a(var1, var2, this.av?0.1F * var9:0.02F);
        
        this.c(this.an, this.ao, this.ap);
        this.ao -= 0.08D;  // 重力
        this.ao *= 0.9800000190734863D;
        this.an *= (double)var8;
        this.ap *= (double)var8;
    }
}
```

**Rust 実装：**
```rust
impl LivingEntity {
    pub fn update_movement(&mut self, move_forward: f32, move_strafing: f32, world: &World) {
        let prev_y = self.entity.position.y;
        
        if self.is_in_water() {
            self.move_in_water(move_forward, move_strafing, 0.02);
        } else if self.is_in_lava() {
            self.move_in_lava(move_forward, move_strafing, 0.02);
        } else {
            self.move_on_land(move_forward, move_strafing, world);
        }
        
        // 重力適用
        self.apply_gravity();
        
        // 衝突検出と位置更新
        self.handle_collisions(world);
        
        // 落下ダメージ処理
        if prev_y - self.entity.position.y > 1.0 {
            self.handle_fall_damage(prev_y - self.entity.position.y);
        }
    }
    
    fn move_on_land(&mut self, move_forward: f32, move_strafing: f32, world: &World) {
        let friction = if self.on_ice(world) { 0.54600006 } else { 0.91 };
        let acceleration_factor = 0.16277136 / (friction * friction * friction);
        let acceleration = if self.on_ice(world) { 
            0.1 * acceleration_factor 
        } else { 
            0.02 
        };
        
        self.apply_movement_input(move_forward, move_strafing, acceleration);
        self.handle_collisions(world);
        
        // 重力
        self.entity.velocity.y -= 0.08;
        self.entity.velocity.y *= 0.98;
        
        // 摩擦
        self.entity.velocity.x *= friction;
        self.entity.velocity.z *= friction;
    }
    
    fn move_in_water(&mut self, move_forward: f32, move_strafing: f32, acceleration: f32) {
        self.apply_movement_input(move_forward, move_strafing, acceleration);
        self.handle_collisions(&World::default());  // 簡略化
        
        // 水中の抵抗
        self.entity.velocity.x *= 0.8;
        self.entity.velocity.y *= 0.8;
        self.entity.velocity.z *= 0.8;
        self.entity.velocity.y -= 0.02;
    }
    
    fn apply_movement_input(&mut self, move_forward: f32, move_strafing: f32, acceleration: f32) {
        let distance = (move_forward * move_forward + move_strafing * move_strafing).sqrt();
        
        if distance > 0.0 {
            let yaw_rad = self.entity.rotation.x.to_radians();
            let sin_yaw = yaw_rad.sin();
            let cos_yaw = yaw_rad.cos();
            
            // 入力をワールド座標に変換
            let move_x = move_forward * cos_yaw - move_strafing * sin_yaw;
            let move_z = move_forward * sin_yaw + move_strafing * cos_yaw;
            
            // 加速度適用
            self.entity.velocity.x += (move_x * acceleration) as f64;
            self.entity.velocity.z += (move_z * acceleration) as f64;
        }
    }
    
    fn apply_gravity(&mut self) {
        if !self.is_in_water() && !self.is_in_lava() && !self.entity.on_ground {
            self.entity.velocity.y -= 0.08;
        }
    }
    
    fn handle_collisions(&mut self, world: &World) {
        // 簡略化された衝突検出
        let new_position = self.entity.position + self.entity.velocity;
        
        if world.is_solid_block(new_position.x, new_position.y, new_position.z) {
            // 衝突処理
            if world.is_solid_block(new_position.x, self.entity.position.y, self.entity.position.z) {
                self.entity.velocity.x = 0.0;
            }
            if world.is_solid_block(self.entity.position.x, new_position.y, self.entity.position.z) {
                self.entity.velocity.y = 0.0;
                self.entity.on_ground = true;
            }
            if world.is_solid_block(self.entity.position.x, self.entity.position.y, new_position.z) {
                self.entity.velocity.z = 0.0;
            }
        } else {
            self.entity.position = new_position;
            self.entity.on_ground = false;
        }
        
        // バウンディングボックス更新
        self.update_bounding_box();
    }
    
    fn update_bounding_box(&mut self) {
        let half_width = self.entity.width / 2.0;
        self.entity.bounding_box = BoundingBox {
            min_x: self.entity.position.x - half_width as f64,
            min_y: self.entity.position.y - self.entity.step_height as f64,
            min_z: self.entity.position.z - half_width as f64,
            max_x: self.entity.position.x + half_width as f64,
            max_y: self.entity.position.y + self.entity.height as f64,
            max_z: self.entity.position.z + half_width as f64,
        };
    }
}
```

### 2.2 ジャンプ処理

```java
// ge.java - ジャンプ処理
protected void C() {
    this.ao = 0.41999998688697815D;  // ジャンプ速度
}
```

**Rust 実装：**
```rust
impl LivingEntity {
    pub fn jump(&mut self) {
        if self.entity.on_ground && !self.is_in_water() && !self.is_in_lava() {
            self.entity.velocity.y = 0.41999998688697815;
            self.is_jumping = true;
        }
    }
    
    pub fn auto_jump(&mut self, world: &World) {
        // 前方にブロックがある場合に自動ジャンプ
        let look_direction = self.get_look_direction();
        let check_distance = 1.0;
        let check_pos = self.entity.position + look_direction * check_distance;
        
        if world.is_solid_block(check_pos.x, check_pos.y + 1.0, check_pos.z) &&
           world.is_solid_block(check_pos.x, check_pos.y, check_pos.z) {
            self.jump();
        }
    }
    
    fn get_look_direction(&self) -> Vec3 {
        let yaw_rad = self.entity.rotation.x.to_radians();
        let pitch_rad = self.entity.rotation.y.to_radians();
        
        Vec3 {
            x: -yaw_rad.sin() * pitch_rad.cos(),
            y: pitch_rad.sin(),
            z: yaw_rad.cos() * pitch_rad.cos(),
        }
    }
}
```

## 3. AI 行動システム

### 3.1 基本的な AI ロジック

Minecraft Alpha のモブは状態ベースのシンプルな AI を使用：

```java
// ge.java - AI 更新処理
protected void b_() {
    ++this.U;  // AI タイマー
    
    // プレイヤー追跡
    dm var1 = this.ag.a(this, -1.0D);
    if(var1 != null) {
        double var2 = var1.ak - this.ak;
        double var4 = var1.al - this.al;
        double var6 = var1.am - this.am;
        double var8 = var2 * var2 + var4 * var4 + var6 * var6;
        
        if(var8 > 16384.0D) {  // 距離が遠すぎる
            this.F();  // 攻撃対象をリセット
        }
        
        // ランダムに攻撃対象を変更
        if(this.U > 600 && this.aQ.nextInt(800) == 0) {
            if(var8 < 1024.0D) {
                this.U = 0;  // タイマーリセット
            } else {
                this.F();  // 攻撃対象をリセット
            }
        }
    }
    
    // ランダム移動
    this.V = 0.0F;
    this.W = 0.0F;
    float var10 = 8.0F;
    
    if(this.aQ.nextFloat() < 0.02F) {  // 2% の確率で新しい目標を設定
        var1 = this.ag.a(this, (double)var10);
        if(var1 != null) {
            this.h = var1;  // 新しい目標を設定
            this.i = 10 + this.aQ.nextInt(20);  // 移動持続時間
        } else {
            this.X = (this.aQ.nextFloat() - 0.5F) * 20.0F;  // ランダムな回転
        }
    }
    
    // 目標への移動
    if(this.h != null) {
        this.b(this.h, 10.0F);  // 目標に向かって移動
        if(this.i-- <= 0 || this.h.aA || this.h.e((kh)this) > (double)(var10 * var10)) {
            this.h = null;  // 目標をクリア
        }
    } else {
        // ランダムな回転移動
        if(this.aQ.nextFloat() < 0.05F) {
            this.X = (this.aQ.nextFloat() - 0.5F) * 20.0F;
        }
        this.aq += this.X;
        this.ar = this.Z;
    }
}
```

**Rust 実装：**
```rust
pub struct MobAI {
    pub ai_timer: i32,
    pub wander_target: Option<EntityId>,
    pub wander_duration: i32,
    pub random_rotation: f32,
    pub move_forward: f32,
    pub move_strafing: f32,
    pub target_entity: Option<EntityId>,
    pub aggression_range: f64,
}

impl MobAI {
    pub fn new() -> Self {
        Self {
            ai_timer: 0,
            wander_target: None,
            wander_duration: 0,
            random_rotation: 0.0,
            move_forward: 0.0,
            move_strafing: 0.0,
            target_entity: None,
            aggression_range: 16.0,
        }
    }
    
    pub fn update(&mut self, entity: &mut LivingEntity, world: &World, entities: &EntityRegistry) {
        self.ai_timer += 1;
        
        // プレイヤー追跡
        self.update_target_tracking(entity, world, entities);
        
        // 移動決定
        self.decide_movement(entity, world, entities);
        
        // 実際の移動適用
        entity.update_movement(self.move_forward, self.move_strafing, world);
    }
    
    fn update_target_tracking(&mut self, entity: &LivingEntity, world: &World, entities: &EntityRegistry) {
        // 最も近いプレイヤーを探す
        if let Some(player_id) = self.find_nearest_player(entity, entities) {
            let player_pos = entities.get_position(player_id);
            let distance = entity.entity.position.distance_squared_to(player_pos);
            
            // 距離チェック
            if distance > 16384.0 {  // 128ブロック以上離れている
                self.target_entity = None;
            }
            
            // ランダムにターゲットを再評価
            if self.ai_timer > 600 && entity.entity.rng.gen_range(0..800) == 0 {
                if distance < 1024.0 {  // 32ブロック以内
                    self.ai_timer = 0;
                } else {
                    self.target_entity = None;
                }
            }
        } else {
            self.target_entity = None;
        }
    }
    
    fn decide_movement(&mut self, entity: &LivingEntity, world: &World, entities: &EntityRegistry) {
        self.move_forward = 0.0;
        self.move_strafing = 0.0;
        
        // ターゲットがいる場合の追跡移動
        if let Some(target_id) = self.target_entity {
            if let Some(target_pos) = entities.get_position(target_id) {
                self.move_towards_position(entity, target_pos, 10.0);
                return;
            }
        }
        
        // ランダムな徘徊
        if entity.entity.rng.gen::<f32>() < 0.02 {  // 2% の確率
            if let Some(wander_pos) = self.find_random_wander_target(entity, world, 8.0) {
                self.wander_target = Some(wander_pos);
                self.wander_duration = 10 + entity.entity.rng.gen_range(0..20);
            } else {
                self.random_rotation = (entity.entity.rng.gen::<f32>() - 0.5) * 20.0;
            }
        }
        
        // 徘徊移動の実行
        if let Some(target_pos) = self.wander_target {
            self.move_towards_position(entity, target_pos, 10.0);
            self.wander_duration -= 1;
            
            // タイムアウトまたは到着でクリア
            if self.wander_duration <= 0 || 
               entity.entity.position.distance_squared_to(target_pos) < 4.0 {
                self.wander_target = None;
            }
        } else {
            // ランダムな回転
            if entity.entity.rng.gen::<f32>() < 0.05 {
                self.random_rotation = (entity.entity.rng.gen::<f32>() - 0.5) * 20.0;
            }
            entity.entity.rotation.x += self.random_rotation;
        }
    }
    
    fn move_towards_position(&mut self, entity: &LivingEntity, target_pos: Vec3, speed: f32) {
        let dx = target_pos.x - entity.entity.position.x;
        let dy = target_pos.y - entity.entity.position.y;
        let dz = target_pos.z - entity.entity.position.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        
        if distance > 0.1 {
            // 目標方向を計算
            let target_yaw = (dx.atan2(dz) * 180.0 / std::f64::consts::PI) as f32 - 90.0;
            let target_pitch = (-dy / distance).atan() as f32 * 180.0 / std::f64::consts::PI;
            
            // 向きを滑らかに変更
            self.smooth_rotation(&mut entity.entity.rotation.x, target_yaw, 10.0);
            self.smooth_rotation(&mut entity.entity.rotation.y, target_pitch, 10.0);
            
            // 前進移動
            self.move_forward = speed;
        }
    }
    
    fn smooth_rotation(&self, current: &mut f32, target: f32, max_change: f32) {
        let mut diff = target - *current;
        
        // 角度の正規化 (-180 to 180)
        while diff > 180.0 { diff -= 360.0; }
        while diff < -180.0 { diff += 360.0; }
        
        // 変更量を制限
        diff = diff.clamp(-max_change, max_change);
        *current += diff;
    }
    
    fn find_nearest_player(&self, entity: &LivingEntity, entities: &EntityRegistry) -> Option<EntityId> {
        let mut nearest_player = None;
        let mut nearest_distance = f64::MAX;
        
        for (&entity_id, entity_data) in entities.iter() {
            if entity_data.is_player {
                let distance = entity.entity.position.distance_squared_to(entity_data.position);
                if distance < nearest_distance && distance < self.aggression_range * self.aggression_range {
                    nearest_distance = distance;
                    nearest_player = Some(entity_id);
                }
            }
        }
        
        nearest_player
    }
    
    fn find_random_wander_target(&self, entity: &LivingEntity, world: &World, radius: f64) -> Option<Vec3> {
        for _ in 0..10 {  // 最大10回試行
            let angle = entity.entity.rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let distance = entity.entity.rng.gen::<f64>() * radius;
            
            let target_x = entity.entity.position.x + angle.cos() * distance;
            let target_z = entity.entity.position.z + angle.sin() * distance;
            let target_y = world.get_ground_height(target_x, target_z);
            
            if world.is_valid_position(target_x, target_y, target_z) {
                return Some(Vec3 { x: target_x, y: target_y, z: target_z });
            }
        }
        
        None
    }
}
```

## 4. 戦闘システム

### 4.1 攻撃ロジック

```java
// ge.java - 攻撃処理
public boolean a(kh var1, int var2) {
    if(this.E <= 0) {  // 体力がない
        return false;
    } else {
        this.R = 1.5F;  // ノックバック耐性
        
        if((float)this.aW > (float)this.j / 2.0F) {  // 無敵時間中
            if(this.F - var2 >= this.E) {
                return false;
            }
            this.E = this.F - var2;
        } else {
            this.F = this.E;
            this.aW = this.j;  // 無敵時間を設定
            this.E -= var2;
            this.G = this.H = 10;  // 衝撃時間
        }
        
        this.I = 0.0F;  // 攻撃方向リセット
        
        if(var1 != null) {
            double var3 = var1.ak - this.ak;
            double var5 = var1.am - this.am;
            
            // ノックバック計算
            for(var5 = (Math.random() - Math.random()) * 0.01D; 
                var3 * var3 + var5 * var5 < 1.0E-4D; 
                var5 = (Math.random() - Math.random()) * 0.01D) {
                var3 = (Math.random() - Math.random()) * 0.01D;
            }
            
            this.I = (float)(Math.atan2(var5, var3) * 180.0D / 3.1415927410125732D) - this.aq;
            this.a(var1, var2, var3, var5);  // ノックバック適用
        } else {
            this.I = (float)((int)(Math.random() * 2.0D) * 180);  // ランダム方向
        }
        
        // 死亡処理
        if(this.E <= 0) {
            this.ag.a(this, this.e(), this.f(), (this.aQ.nextFloat() - this.aQ.nextFloat()) * 0.2F + 1.0F);
            this.b(var1);  // ドロップアイテム処理
        } else {
            this.ag.a(this, this.d(), this.f(), (this.aQ.nextFloat() - this.aQ.nextFloat()) * 0.2F + 1.0F);
        }
        
        return true;
    }
}
```

**Rust 実装：**
```rust
impl LivingEntity {
    pub fn attack_entity(&mut self, target: &mut LivingEntity, damage: i32) -> bool {
        if self.current_health <= 0 {
            return false;
        }
        
        // ノックバック耐性をリセット
        self.attack_direction = 0.0;
        
        // ダメージ適用
        if target.entity.hurt_time > target.max_health / 2 {
            // 無敵時間中はダメージを無効化
            if target.prev_health - damage >= target.current_health {
                return false;
            }
            target.current_health = target.prev_health - damage;
        } else {
            target.prev_health = target.current_health;
            target.entity.hurt_time = target.max_health;  // 無敵時間
            target.current_health -= damage;
            target.death_timer = 10;  // 衝撃時間
        }
        
        // ノックバック計算
        let knockback_force = 0.4;
        let dx = target.entity.position.x - self.entity.position.x;
        let dz = target.entity.position.z - self.entity.position.z;
        
        let (knockback_x, knockback_z) = if dx * dx + dz * dz < 1e-4 {
            // 距離が近すぎる場合はランダムな方向
            let angle = target.entity.rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            (angle.cos() * knockback_force, angle.sin() * knockback_force)
        } else {
            let distance = (dx * dx + dz * dz).sqrt();
            (dx / distance * knockback_force, dz / distance * knockback_force)
        };
        
        // ノックバック適用
        target.entity.velocity.x += knockback_x;
        target.entity.velocity.y += 0.4;  // 上方向へのノックバック
        target.entity.velocity.z += knockback_z;
        
        // 攻撃方向を設定
        target.attack_direction = ((dz.atan2(dx) * 180.0 / std::f64::consts::PI) as f32 - target.entity.rotation.x);
        
        // 死亡チェック
        if target.current_health <= 0 {
            target.handle_death();
        } else {
            target.play_hurt_sound();
        }
        
        true
    }
    
    fn handle_death(&mut self) {
        self.entity.is_dead = true;
        self.death_timer = 20;  // 死亡アニメーション時間
        
        // ドロップアイテム生成
        self.drop_items();
        
        // 死亡音
        self.play_death_sound();
    }
    
    fn drop_items(&mut self) {
        // 経験値オーブのドロップ
        let exp_amount = self.entity.rng.gen_range(1..=3);
        for _ in 0..exp_amount {
            // 経験値オーブを生成
            self.spawn_experience_orb();
        }
        
        // 特定のアイテムドロップ
        self.drop_specific_items();
    }
    
    fn drop_specific_items(&mut self) {
        // モブ固有のドロップロジック
        // 例：豚なら生肉、羊なら羊毛など
    }
    
    fn play_hurt_sound(&self) {
        // ダメージ音の再生
    }
    
    fn play_death_sound(&self) {
        // 死亡音の再生
    }
}
```

### 4.2 戦闘 AI

```rust
pub struct CombatAI {
    pub attack_cooldown: i32,
    pub attack_range: f64,
    pub max_attack_cooldown: i32,
    pub retreat_threshold: f32,  // 体力割合で逃走
}

impl CombatAI {
    pub fn new() -> Self {
        Self {
            attack_cooldown: 0,
            attack_range: 3.0,
            max_attack_cooldown: 20,
            retreat_threshold: 0.2,
        }
    }
    
    pub fn update_combat(&mut self, mob: &mut LivingEntity, target: &mut LivingEntity) {
        if self.attack_cooldown > 0 {
            self.attack_cooldown -= 1;
        }
        
        let distance = mob.entity.position.distance_to(target.entity.position);
        
        // 攻撃範囲内で攻撃可能
        if distance <= self.attack_range && self.attack_cooldown == 0 {
            let damage = self.calculate_damage(mob, target);
            mob.attack_entity(target, damage);
            self.attack_cooldown = self.max_attack_cooldown;
        }
        
        // 体力が少ない場合は逃走
        if mob.current_health as f32 <= mob.max_health as f32 * self.retreat_threshold {
            self.retreat_from_target(mob, target);
        }
    }
    
    fn calculate_damage(&self, attacker: &LivingEntity, target: &LivingEntity) -> i32 {
        // 基本ダメージにランダム要素を追加
        let base_damage = 2;
        let random_factor = attacker.entity.rng.gen_range(0..=2);
        base_damage + random_factor
    }
    
    fn retreat_from_target(&mut self, mob: &mut LivingEntity, target: &LivingEntity) {
        // ターゲットから離れる方向に移動
        let dx = mob.entity.position.x - target.entity.position.x;
        let dz = mob.entity.position.z - target.entity.position.z;
        let distance = (dx * dx + dz * dz).sqrt();
        
        if distance > 0.1 {
            // 逃走方向を設定
            let retreat_yaw = (dx.atan2(dz) * 180.0 / std::f64::consts::PI) as f32 - 90.0;
            mob.entity.rotation.x = retreat_yaw;
            
            // 速度を上げて逃走
            mob.move_forward = 1.5;  // 通常より速い移動
        }
    }
}
```

## 5. 特定モブの実装例

### 5.1 ゾンビの実装

```rust
pub struct Zombie {
    pub living_entity: LivingEntity,
    pub combat_ai: CombatAI,
    pub mob_ai: MobAI,
}

impl Zombie {
    pub fn new(position: Vec3) -> Self {
        let mut living_entity = LivingEntity {
            entity: Entity {
                position,
                velocity: Vec3::zero(),
                rotation: Vec2 { x: 0.0, y: 0.0 },
                width: 0.6,
                height: 1.8,
                health: 20,
                ..Default::default()
            },
            max_health: 20,
            current_health: 20,
            move_speed: 0.2,
            ..Default::default()
        };
        
        Self {
            living_entity,
            combat_ai: CombatAI {
                attack_range: 2.0,
                max_attack_cooldown: 30,
                ..Default::default()
            },
            mob_ai: MobAI {
                aggression_range: 32.0,
                ..Default::default()
            },
        }
    }
    
    pub fn update(&mut self, world: &World, entities: &EntityRegistry) {
        // 基本的な AI 更新
        self.mob_ai.update(&mut self.living_entity, world, entities);
        
        // 戦闘 AI 更新
        if let Some(target_id) = self.mob_ai.target_entity {
            if let Some(target) = entities.get_mut_living(target_id) {
                self.combat_ai.update_combat(&mut self.living_entity, target);
            }
        }
        
        // ゾンビ固有の行動
        self.update_zombie_behavior(world);
    }
    
    fn update_zombie_behavior(&mut self, world: &World) {
        // 夜間のみ活動的
        if world.is_daytime() {
            // 日中は炎上ダメージ
            if world.is_exposed_to_sunlight(self.living_entity.entity.position) {
                self.living_entity.take_fire_damage(1);
            }
        }
        
        // 特殊な移動（ドア破壊など）
        if self.living_entity.entity.rng.gen::<f32>() < 0.001 {
            self.try_break_door(world);
        }
    }
    
    fn try_break_door(&mut self, world: &World) {
        // 前方のドアを探して破壊
        let look_dir = self.living_entity.get_look_direction();
        let door_pos = self.living_entity.entity.position + look_dir * 2.0;
        
        if world.is_block_at(door_pos.x, door_pos.y, door_pos.z, BlockType::Door) {
            world.break_block(door_pos.x, door_pos.y, door_pos.z);
        }
    }
}
```

### 5.2 豚の実装

```rust
pub struct Pig {
    pub living_entity: LivingEntity,
    pub mob_ai: MobAI,
    pub panic_timer: i32,
}

impl Pig {
    pub fn new(position: Vec3) -> Self {
        let mut living_entity = LivingEntity {
            entity: Entity {
                position,
                width: 0.9,
                height: 0.9,
                health: 10,
                ..Default::default()
            },
            max_health: 10,
            current_health: 10,
            move_speed: 0.25,
            ..Default::default()
        };
        
        Self {
            living_entity,
            mob_ai: MobAI {
                aggression_range: 0.0,  // 攻撃しない
                ..Default::default()
            },
            panic_timer: 0,
        }
    }
    
    pub fn update(&mut self, world: &World, entities: &EntityRegistry) {
        // パニック状態チェック
        if self.panic_timer > 0 {
            self.panic_timer -= 1;
            self.panic_movement(world);
        } else {
            // 通常の徘徊
            self.mob_ai.update(&mut self.living_entity, world, entities);
        }
        
        // 脅威検出
        self.detect_threats(entities);
    }
    
    fn detect_threats(&mut self, entities: &EntityRegistry) {
        // 近くのプレイヤーを検出してパニック
        for (&entity_id, entity_data) in entities.iter() {
            if entity_data.is_player {
                let distance = self.living_entity.entity.position.distance_to(entity_data.position);
                if distance < 8.0 {
                    self.panic_timer = 100;  // パニック状態
                    break;
                }
            }
        }
    }
    
    fn panic_movement(&mut self, world: &World) {
        // ランダムな方向に速く移動
        if self.living_entity.entity.rng.gen::<f32>() < 0.1 {
            let random_yaw = self.living_entity.entity.rng.gen::<f32>() * 360.0;
            self.living_entity.entity.rotation.x = random_yaw;
        }
        
        // 速い移動
        self.living_entity.move_forward = 1.5;
        
        // ジャンプしやすく
        if self.living_entity.entity.rng.gen::<f32>() < 0.3 {
            self.living_entity.jump();
        }
    }
    
    fn drop_items(&mut self) {
        // 豚は生肉をドロップ
        let meat_count = self.living_entity.entity.rng.gen_range(1..=3);
        for _ in 0..meat_count {
            self.spawn_item(ItemType::RawPorkchop);
        }
    }
}
```

## 6. パフォーマンス最適化

### 6.1 エンティティ管理の最適化

```rust
pub struct EntityRegistry {
    entities: HashMap<EntityId, EntityData>,
    living_entities: HashMap<EntityId, LivingEntity>,
    next_id: EntityId,
}

pub struct EntityData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub is_player: bool,
    pub entity_type: EntityType,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            living_entities: HashMap::new(),
            next_id: 0,
        }
    }
    
    pub fn spawn_entity(&mut self, entity: LivingEntity) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        
        let entity_data = EntityData {
            position: entity.entity.position,
            velocity: entity.entity.velocity,
            is_player: false,
            entity_type: EntityType::from(&entity),
        };
        
        self.entities.insert(id, entity_data);
        self.living_entities.insert(id, entity);
        
        id
    }
    
    pub fn update_all_entities(&mut self, world: &World) {
        // 並列処理によるパフォーマンス向上
        use rayon::prelude::*;
        
        let entities: Vec<_> = self.living_entities.par_iter_mut()
            .filter(|(_, entity)| !entity.entity.is_dead)
            .collect();
        
        for (id, entity) in entities {
            // エンティティごとの更新処理
            self.update_single_entity(*id, entity, world);
        }
        
        // 死亡したエンティティを削除
        self.living_entities.retain(|_, entity| !entity.entity.is_dead);
        self.entities.retain(|id, _| self.living_entities.contains_key(id));
    }
    
    fn update_single_entity(&mut self, id: EntityId, entity: &mut LivingEntity, world: &World) {
        // エンティティタイプに応じた更新処理
        match self.entities.get(&id).unwrap().entity_type {
            EntityType::Zombie => {
                // ゾンビの更新処理
            }
            EntityType::Pig => {
                // 豚の更新処理
            }
            EntityType::Player => {
                // プレイヤーの更新処理
            }
        }
    }
}
```

## 7. まとめ

Minecraft Alpha のモブ行動システムの特徴：

1. **シンプルな状態ベース AI**: 複雑なパスファインディングではなく、基本的な追跡と徘徊
2. **物理ベースの移動**: 重力、摩擦、衝突検出を考慮したリアルな移動
3. **ランダム要素**: 行動の多様性を生み出す確率的な決定
4. **戦闘システム**: ダメージ、ノックバック、死亡処理の統合
5. **モブ固有の行動**: 各モブの特性に応じた特殊な振る舞い

このアルゴリズムを Rust で実装することで、効率的でスケーラブルなモブシステムが構築できます。特に並列処理とメモリ管理の面で Rust の強みを活かせます。
