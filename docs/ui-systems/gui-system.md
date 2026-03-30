# GUIシステム分析

## GuiScreen（bh）クラス

### クラス概要
**ファイル**: `bh.java`  
**役割**: GUI画面基底クラス、入力処理、ボタン管理

### 主要なフィールド変数
```java
protected Minecraft b;  // Minecraftインスタンス
public int c, d;        // 画面サイズ
protected List e = new ArrayList();  // ボタンリスト
public boolean f = false;  // 描画フラグ
protected kd g;        // フォントレンダラー
private fk a = null;   // 選択中ボタン
```

### GUI初期化
```java
public void a(Minecraft var1, int var2, int var3) {
    this.b = var1;
    this.g = var1.o;  // フォントレンダラー取得
    this.c = var2;    // 幅
    this.d = var3;    // 高さ
    this.a();         // 初期化処理
}
```

### 入力処理
```java
// マウス入力
public void d() {
    while(Mouse.next()) {
        this.e();  // マウスイベント処理
    }
    
    while(Keyboard.next()) {
        this.f();  // キーボードイベント処理
    }
}

// ボタンクリック
protected void a(int var1, int var2, int var3) {
    if(var3 == 0) {  // 左クリック
        for(int i = 0; i < this.e.size(); i++) {
            Button button = (Button)this.e.get(i);
            if(button.c(this.b, var1, var2)) {
                this.a = button;  // 選択中ボタン設定
                this.b.A.a("random.click", 1.0f, 1.0f);
                this.a(button);  // ボタンアクション
            }
        }
    }
}

// ボタン解放
protected void b(int var1, int var2, int var3) {
    if(this.a != null && var3 == 0) {
        this.a.a(var1, var2);  // ボタン解放処理
        this.a = null;
    }
}
```

### GUIコンポーネント
```java
// ボタン描画
protected void a(fk var1) {}  // ボタン描画（サブクラスで実装）

// テキスト入力
protected void a(char var1, int var2) {
    if(var2 == 1) {  // ESCキー
        this.b.a((bh)null);  // GUIを閉じる
        this.b.e();  // ゲームに戻る
    }
}
```

## HUD（lu）クラス

### クラス概要
**ファイル**: `lu.java`  
**役割**: HUDレンダラー、ホットバー、体力表示、クロスヘア

### 主要なフィールド変数
```java
private static ab d = new ab();  // アイテムレンダラー
private List e = new ArrayList();  // 通知リスト
private Random f = new Random();   // 乱数生成器
private Minecraft g;              // Minecraftインスタンス
public String a = null;           // デバッグ情報
private int h = 0;               // 更新カウンタ
private String i = "";           // チャットメッセージ
public float b;                  // 画面揺れ
float c = 1.0f;                // 明るさ
```

### HUD描画
```java
public void a(float var1, boolean var2, int var3, int var4) {
    // 画面サイズ取得
    iy screen = new iy(this.g.c, this.g.d);
    int width = screen.a();
    int height = screen.b();
    
    // デバッグ情報表示
    if(this.g.y.i) {
        this.a(this.g.g.a(var1), width, height);
    }
    
    // GUIテクスチャバインド
    GL11.glColor4f(1.0f, 1.0f, 1.0f, 1.0f);
    GL11.glBindTexture(3553, this.g.n.a("/gui/gui.png"));
    
    // ホットバー描画
    eu inventory = this.g.g.b;
    this.k = -90.0f;
    this.b(width / 2 - 91, height - 22, 0, 0, 182, 22);
    
    // 選択中アイテム表示
    this.b(width / 2 - 91 - 1 + inventory.d * 20, height - 22 - 1, 0, 22, 24, 22);
    
    // アイテムアイコン
    GL11.glBindTexture(3553, this.g.n.a("/gui/icons.png"));
    GL11.glEnable(3042);
    GL11.glBlendFunc(775, 769);
    this.b(width / 2 - 7, height / 2 - 7, 0, 0, 16, 16);
    GL11.glDisable(3042);
}
```

### ホットバー描画
```java
// ホットバーのアイテム描画
boolean isOffhand = this.g.g.aW / 3 % 2 == 1;
int selectedSlot = this.g.g.E;
int hotbarSlot = this.g.g.F;

this.f.setSeed((long)(this.h * 312871));

for(int i = 0; i < 10; i++) {
    int y = height / 2 - 32;
    int slotIndex = this.g.g.m();  // インベントリスロット
    
    if(slotIndex > 0) {
        int x = width / 2 + 91 - i * 8 - 9;
        
        // アイテム描画
        if(i * 2 + 1 < slotIndex) {
            this.b(x, y, 34, 9, 9, 9);
        }
        
        if(i * 2 + 1 == slotIndex) {
            this.b(x, y, 25, 9, 9, 9);
        }
        
        if(i * 2 + 1 > slotIndex) {
            this.b(x, y, 16, 9, 9, 9);
        }
    }
    
    // オフハンド表示
    byte offhandSlot = 0;
    if(isOffhand) {
        offhandSlot = 1;
    }
    
    int itemX = width / 2 - 91 + i * 8;
    if(selectedSlot <= 4) {
        y += this.f.nextInt(2);
    }
    
    this.b(itemX, y, 16 + offhandSlot * 9, 0, 9, 9);
    
    // 耐久度表示
    if(selectedSlot <= 4) {
        y += this.f.nextInt(2);
    }
    
    if(isOffhand) {
        if(i * 2 + 1 < hotbarSlot) {
            this.b(itemX, y, 70, 0, 9, 9);
        }
        
        if(i * 2 + 1 == hotbarSlot) {
            this.b(itemX, y, 79, 0, 9, 9);
        }
        
        if(i * 2 + 1 > hotbarSlot) {
            this.b(itemX, y, 16, 0, 9, 9);
        }
    }
}
```

## GUI画面の種類

### メインメニュー（cx）
```java
public class cx extends bh {
    private static final Random h = new Random();
    String[] a = new String[]{
        " *   * * *   * *** *** *** *** ***", 
        " ** ** * **  * *   *   * * * * *    * ", 
        " * * * * * * * **  *   **  *** **   * "
    };
    
    // Minecraftロゴ描画
    // スプラッシュテキスト表示
    // メニューボタン（Singleplayer, Multiplayer, etc.）
}
```

### オプション画面（ay）
```java
public class ay extends bh {
    private bh h;  // 親画面
    protected String a = "Options";
    
    // グラフィック設定、音量設定など
    // 各種オプションの調整
}
```

### ワールド選択画面（jq）
```java
public class jq extends bh {
    protected bh a;
    protected String h = "Select world";
    
    // ワールドリスト表示
    // ワールド作成・削除
    // ワールド読み込み
}
```

### チャット画面（as）
```java
public class as extends bh {
    private String a;
    private String h;
    
    // チャットメッセージ表示
    // チャット入力処理
    // コマンド実行
}
```

## ボタンシステム

### Button（fk）クラス
```java
public class fk {
    public int a;  // ボタンID
    public int b;  // X座標
    public int c;  // Y座標
    public int d;  // 幅
    public int e;  // 高さ
    public String f;  // ボタンテキスト
    
    // ボタンクリック判定
    public boolean c(Minecraft var1, int var2, int var3) {
        return var2 >= this.b && var3 >= this.c && 
               var2 < this.b + this.d && 
               var3 < this.c + this.e;
    }
}
```

### ボタン描画
```java
// ボタン背景描画
protected void drawButton(Button button) {
    // ボタン背景
    this.b(button.b, button.c, 0, 46, button.d, button.e);
    
    // ボタンテキスト
    this.g.a(button.f, 
              button.b + button.d / 2, 
              button.c + (button.e - 8) / 2, 
              0xFFFFFF);
}
```

## 入力処理

### キーボード入力
```java
// キー押下処理
public void f() {
    char keyChar = Keyboard.getEventCharacter();
    int keyCode = Keyboard.getEventKey();
    
    if(Keyboard.getEventKeyState()) {
        // キー押下
        this.a(keyChar, keyCode);
    } else {
        // キー解放
        // 必要に応じて処理
    }
}
```

### マウス入力
```java
// マウスイベント処理
public void e() {
    int mouseX = Mouse.getEventX();
    int mouseY = Mouse.getEventY();
    int button = Mouse.getEventButton();
    boolean buttonState = Mouse.getEventButtonState();
    
    if(buttonState) {
        // マウス押下
        this.a(mouseX, mouseY, button);
    } else {
        // マウス解放
        this.b(mouseX, mouseY, button);
    }
}
```

## テキストレンダリング

### FontRenderer（kd）クラス
```java
public class kd {
    // テキスト描画
    public void a(String text, float x, float y, int color) {
        // 文字ごとに描画
        for(int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            this.drawChar(c, x + i * 8, y, color);
        }
    }
    
    // 文字幅計算
    public int a(String text) {
        return text.length() * 8;  // 固定幅フォント
    }
}
```

## Rust移植時の設計提案

### GUIトレイト
```rust
pub trait GuiScreen {
    fn init(&mut self, minecraft: &mut Minecraft, width: i32, height: i32);
    fn render(&mut self, renderer: &mut GuiRenderer, mouse_x: i32, mouse_y: i32, delta_time: f32);
    fn handle_mouse_event(&mut self, event: MouseEvent);
    fn handle_keyboard_event(&mut self, event: KeyboardEvent);
    fn should_pause_game(&self) -> bool;
}

pub trait Button {
    fn get_id(&self) -> i32;
    fn get_bounds(&self) -> Rect<i32>;
    fn is_hovered(&self, mouse_x: i32, mouse_y: i32) -> bool;
    fn render(&self, renderer: &mut GuiRenderer, hovered: bool);
    fn on_click(&mut self, screen: &mut dyn GuiScreen);
}
```

### GUIシステム
```rust
pub struct GuiManager {
    current_screen: Option<Box<dyn GuiScreen>>,
    screens: HashMap<String, Box<dyn GuiScreen>>,
    font_renderer: FontRenderer,
    texture_manager: TextureManager,
}

impl GuiManager {
    pub fn new(texture_manager: TextureManager) -> Self {
        Self {
            current_screen: None,
            screens: HashMap::new(),
            font_renderer: FontRenderer::new(texture_manager.clone()),
            texture_manager,
        }
    }
    
    pub fn set_screen(&mut self, screen: Box<dyn GuiScreen>) {
        self.current_screen = Some(screen);
    }
    
    pub fn render(&mut self, renderer: &mut GuiRenderer, delta_time: f32) {
        if let Some(ref mut screen) = self.current_screen {
            screen.render(renderer, 0, 0, delta_time);
        }
    }
    
    pub fn handle_input(&mut self, event: InputEvent) {
        if let Some(ref mut screen) = self.current_screen {
            match event {
                InputEvent::Mouse(mouse_event) => {
                    screen.handle_mouse_event(mouse_event);
                }
                InputEvent::Keyboard(keyboard_event) => {
                    screen.handle_keyboard_event(keyboard_event);
                }
            }
        }
    }
}
```

### HUDシステム
```rust
pub struct HudRenderer {
    font: FontRenderer,
    item_renderer: ItemRenderer,
    texture_manager: TextureManager,
    debug_info: bool,
}

impl HudRenderer {
    pub fn new(texture_manager: TextureManager) -> Self {
        Self {
            font: FontRenderer::new(texture_manager.clone()),
            item_renderer: ItemRenderer::new(texture_manager.clone()),
            texture_manager,
            debug_info: false,
        }
    }
    
    pub fn render(&mut self, player: &Player, renderer: &mut Renderer, delta_time: f32) {
        self.render_hotbar(player, renderer);
        self.render_crosshair(renderer);
        
        if self.debug_info {
            self.render_debug_info(player, renderer);
        }
    }
    
    fn render_hotbar(&self, player: &Player, renderer: &mut Renderer) {
        let inventory = player.get_inventory();
        let selected_slot = inventory.get_selected_hotbar();
        
        // ホットバー背景
        renderer.draw_texture_rect(
            &self.texture_manager.get_texture("gui/gui.png"),
            Rect::new(0, 0, 182, 22),
            Rect::centered_x(182, -22)
        );
        
        // アイテム描画
        for i in 0..10 {
            if let Some(item) = inventory.get_hotbar_item(i) {
                let x = -91 + i * 20;
                let y = -22;
                
                self.item_renderer.render_item(
                    item,
                    Vec2::new(x, y),
                    renderer
                );
            }
        }
        
        // 選択ハイライト
        let highlight_x = -91 + selected_slot * 20;
        renderer.draw_texture_rect(
            &self.texture_manager.get_texture("gui/icons.png"),
            Rect::new(0, 22, 24, 22),
            Rect::new(highlight_x - 1, -23, 24, 22)
        );
    }
    
    fn render_crosshair(&self, renderer: &mut Renderer) {
        renderer.draw_texture_rect(
            &self.texture_manager.get_texture("gui/icons.png"),
            Rect::new(0, 0, 16, 16),
            Rect::centered(16, 16)
        );
    }
    
    fn render_debug_info(&self, player: &Player, renderer: &mut Renderer) {
        let pos = player.get_position();
        let debug_text = format!(
            "XYZ: {:.2}/{:.2}/{:.2}\nFPS: {}",
            pos.x, pos.y, pos.z,
            renderer.get_fps()
        );
        
        self.font.render_multiline(
            &debug_text,
            Vec2::new(2, 2),
            Color::WHITE,
            renderer
        );
    }
}
```

### ボタンシステム
```rust
pub struct Button {
    id: i32,
    bounds: Rect<i32>,
    text: String,
    on_click: Option<Box<dyn Fn(&mut dyn GuiScreen)>>,
}

impl Button {
    pub fn new(id: i32, x: i32, y: i32, width: i32, height: i32, text: &str) -> Self {
        Self {
            id,
            bounds: Rect::new(x, y, width, height),
            text: text.to_string(),
            on_click: None,
        }
    }
    
    pub fn with_callback<F>(mut self, callback: F) -> Self 
    where 
        F: Fn(&mut dyn GuiScreen) + 'static
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl ButtonTrait for Button {
    fn get_id(&self) -> i32 { self.id }
    fn get_bounds(&self) -> Rect<i32> { self.bounds }
    fn is_hovered(&self, mouse_x: i32, mouse_y: i32) -> bool {
        self.bounds.contains(mouse_x, mouse_y)
    }
    fn render(&self, renderer: &mut GuiRenderer, hovered: bool) {
        let tex_y = if hovered { 46 } else { 66 };
        
        renderer.draw_texture_rect(
            &renderer.get_texture("gui/gui.png"),
            Rect::new(0, tex_y, 200, 20),
            self.bounds
        );
        
        renderer.draw_text_centered(
            &self.text,
            self.bounds.center(),
            Color::WHITE
        );
    }
    
    fn on_click(&mut self, screen: &mut dyn GuiScreen) {
        if let Some(ref callback) = self.on_click {
            callback(screen);
        }
    }
}
```

このGUIシステムはMinecraftのユーザーインターフェースの中核であり、効率的な描画と直感的な入力処理が重要です。
