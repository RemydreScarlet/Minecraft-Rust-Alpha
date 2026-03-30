# ネットワークシステム分析

## 概要

Minecraft Alpha v1.1.2_01のネットワークシステムは、クライアント-サーバー間の通信を管理する重要なコンポーネントです。パケットベースの通信プロトコルを使用し、各種ゲームデータの同期を行います。

## Packet（fn）基底クラス

### クラス概要
**ファイル**: `fn.java`  
**役割**: ネットワークパケットの基底クラス、パケット登録・シリアライズ

### 主要な機能
```java
public abstract class fn {
    private static Map a = new HashMap();  // ID→クラスマップ
    private static Map b = new HashMap();  // クラス→IDマップ
    public boolean j = false;             // 圧縮フラグ
    
    // パケット登録
    static void a(int var0, Class var1) {
        a.put(Integer.valueOf(var0), var1);
        b.put(var1, Integer.valueOf(var0));
    }
    
    // パケット生成
    public static fn a(int var0) {
        Class var1 = (Class)a.get(Integer.valueOf(var0));
        return var1 == null ? null : (fn)var1.newInstance();
    }
    
    // パケット読み込み
    public static fn b(DataInputStream var0) {
        int packetId = var0.read();
        if(packetId == -1) {
            return null;
        } else {
            fn packet = a(packetId);
            if(packet == null) {
                throw new IOException("Bad packet id " + packetId);
            } else {
                packet.a(packetId);
                return packet;
            }
        }
    }
}
```

### パケットID登録
```java
static {
    a(0, gi.class);    // KeepAlive
    a(1, hp.class);    // Login
    a(2, gt.class);    // Handshake
    a(3, ij.class);    // Chat Message
    a(4, du.class);    // Time Update
    a(5, m.class);    // Entity Equipment
    a(6, ji.class);    // Spawn Position
    a(10, eh.class);   // Use Entity
    a(11, s.class);    // Update Health
    a(12, mh.class);   // Respawn
    a(13, ch.class);   // Player Position
    a(14, fg.class);   // Player Look
    a(15, do.class);   // Player Position & Look
    a(16, dz.class);   // Player Digging
    a(17, ld.class);   // Player Block Placement
    a(18, hf.class);   // Holding Change
    a(20, gp.class);   // Animation
    a(21, ha.class);   // Entity Action
    a(22, bm.class);   // Named Entity Spawn
    a(23, kj.class);   // Pickup Spawn
    a(24, ez.class);   // Collect Item
    a(29, ju.class);   // Mob Spawn
    a(30, lq.class);   // Entity Painting
    a(31, kp.class);   // Entity Velocity
    a(32, jx.class);   // Destroy Entity
    a(33, is.class);   // Entity
    a(34, jl.class);   // Entity Relative Move
    a(50, ka.class);   // Update Sign
    a(51, bz.class);   // Update Tile Entity
    a(52, na.class);   // Player List Item
    a(53, li.class);   // Player List
    a(59, ny.class);   // Explosion
    a(255, oh.class);  // Disconnect
}
```

## 主要なパケットクラス

### KeepAlive（gi）
```java
public class gi extends fn {
    public void a(lb var1) {}  // 読み込み処理なし
    
    public void a(lb var1) {
        var1.b(this.b());  // パケットID書き込み
    }
    
    public int a() {
        return 0;  // KeepAliveパケットID
    }
}
```

### Login（hp）
```java
public class hp extends fn {
    public String a;  // プレイヤー名
    public String b;  // 接続タイプ
    
    public void a(lb var1) {
        this.a = var1.c(16);  // 文字列読み込み
        this.b = var1.c(16);
    }
    
    public void a(lb var1) {
        var1.b(this.b());
        var1.a(this.a);
        var1.a(this.b);
    }
    
    public int a() {
        return 1;
    }
}
```

### Chat Message（ij）
```java
public class ij extends fn {
    public String a;  // チャットメッセージ
    
    public void a(lb var1) {
        this.a = var1.c(16);
    }
    
    public void a(lb var1) {
        var1.b(this.b());
        var1.a(this.a);
    }
    
    public int a() {
        return 3;
    }
}
```

### Player Position（eh）
```java
public class eh extends fn {
    public double a;  // X座標
    public double b;  // Y座標
    public double c;  // Z座標
    public boolean d;  // 地面接触
    public boolean e;  // 地面接触
    
    public void a(lb var1) {
        this.a = var1.readDouble();
        this.b = var1.readDouble();
        this.c = var1.readDouble();
        this.d = var1.readBoolean();
        this.e = var1.readBoolean();
    }
    
    public void a(lb var1) {
        var1.b(this.b());
        var1.writeDouble(this.a);
        var1.writeDouble(this.b);
        var1.writeDouble(this.c);
        var1.writeBoolean(this.d);
        var1.writeBoolean(this.e);
    }
    
    public int a() {
        return 13;
    }
}
```

### Entity Velocity（kp）
```java
public class kp extends fn {
    public int a;    // エンティティID
    public short b;   // X方向速度
    public short c;   // Y方向速度
    public short d;   // Z方向速度
    
    public void a(lb var1) {
        this.a = var1.readInt();
        this.b = var1.readShort();
        this.c = var1.readShort();
        this.d = var1.readShort();
    }
    
    public void a(lb var1) {
        var1.b(this.b());
        var1.writeInt(this.a);
        var1.writeShort(this.b);
        var1.writeShort(this.c);
        var1.writeShort(this.d);
    }
    
    public int a() {
        return 31;
    }
}
```

## ネットワークマネージャ

### パケット処理フロー
```java
// パケット受信
public void handlePacket(DataInputStream inputStream) {
    try {
        fn packet = fn.b(inputStream);
        if(packet != null) {
            packet.a(this);  // パケット処理
        }
    } catch(IOException e) {
        System.err.println("Failed to handle packet: " + e.getMessage());
    }
}

// パケット送信
public void sendPacket(fn packet, DataOutputStream outputStream) {
    try {
        fn.a(packet, outputStream);
    } catch(IOException e) {
        System.err.println("Failed to send packet: " + e.getMessage());
    }
}
```

### 圧縮処理
```java
// 圧縮されたパケット
public class bz extends fn {
    public int a;    // 圧縮サイズ
    public int b;    // 非圧縮サイズ
    
    public void a(lb var1) {
        this.a = var1.readInt();
        this.b = var1.readInt();
        
        // データ展開
        byte[] compressed = new byte[this.a];
        var1.readFully(compressed);
        
        Inflater inflater = new Inflater();
        inflater.setInput(compressed);
        byte[] decompressed = new byte[this.b];
        inflater.inflate(decompressed);
        
        // 展開データからパケット読み込み
        DataInputStream dis = new DataInputStream(new ByteArrayInputStream(decompressed));
        fn packet = fn.b(dis);
        packet.a(this);
    }
}
```

## 外部ライブラリ

### JOrbis Oggデコーダ
**ファイル**: `com/jcraft/jorbis/`  
**役割**: Ogg Vorbisオーディオフォーマットのデコード

#### 主要クラス
```java
// Packetクラス - Oggパケット処理
public class Packet {
    public byte[] packet_base;  // パケットデータ
    public int packet;          // パケット番号
    public int bytes;           // バイト数
    public int b_o_s;          // 開始オフセット
    public int e_o_s;          // 終了オフセット
    public long granulepos;     // グラニュール位置
    public long packetno;       // パケット番号
}

// Commentクラス - Oggコメント処理
public class Comment {
    // コメントの読み込み・書き込み
    public int header_out(Packet packet);
    public int synthesis_headerin(Comment comment, Packet packet);
}

// Infoクラス - Ogg情報処理
public class Info {
    private static final int OV_EBADPACKET = -136;
    private static final int OV_ENOTAUDIO = -135;
    
    // デコード情報管理
    public int synthesis_headerin(Comment comment, Packet packet);
    public int blocksize(Packet packet);
}
```

### Paulscode Sound System
**ファイル**: `paulscode/sound/`  
**役割**: 3Dオーディオ、サウンド再生管理

#### 主要機能
```java
// サウンドシステム初期化
SoundSystemConfig.setLibrary("LWJG");
SoundSystemConfig.addLibrary(LibraryLWJGOL.class);

// サウンド読み込み
soundSystem.newSource(false, "background", "background.ogg", true);
soundSystem.setVolume("background", 0.8f);

// 3Dポジション指定再生
soundSystem.play("effect", 1.0f, 1.0f, 1.0f, 
                   SoundSystemConfig.ATTENUATION_ROLLOFF, 
                   SoundSystemConfig.DEFAULT_ROLLOFF);
```

## Rust移植時の設計提案

### パケットトレイト
```rust
pub trait Packet {
    fn get_id(&self) -> u8;
    fn serialize(&self, buf: &mut Vec<u8>);
    fn deserialize(&mut self, buf: &[u8]) -> Result<(), PacketError>;
    fn handle(&self, connection: &mut NetworkConnection);
}

#[derive(Debug)]
pub enum PacketError {
    InvalidPacketId(u8),
    InvalidData,
    BufferTooSmall,
    IoError(std::io::Error),
}
```

### パケットマクロ
```rust
// パケット登録マクロ
macro_rules! register_packets {
    ($($id:expr => $packet_type:ty),*) => {
        pub fn create_packet(id: u8, data: &[u8]) -> Result<Box<dyn Packet>, PacketError> {
            match id {
                $(
                    $id => {
                        let mut packet = <$packet_type>::new();
                        packet.deserialize(data)?;
                        Ok(Box::new(packet))
                    }
                )*
                _ => Err(PacketError::InvalidPacketId(id))
            }
        }
    };
}

// パケット実装マクロ
macro_rules! packet_impl {
    ($struct_name:ident, $packet_id:expr) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            // フィールド定義
        }
        
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    // 初期化
                }
            }
        }
        
        impl Packet for $struct_name {
            fn get_id(&self) -> u8 { $packet_id }
            
            fn serialize(&self, buf: &mut Vec<u8>) {
                // シリアライズ処理
            }
            
            fn deserialize(&mut self, buf: &[u8]) -> Result<(), PacketError> {
                // デシリアライズ処理
            }
            
            fn handle(&self, connection: &mut NetworkConnection) {
                // パケット処理
            }
        }
    };
}
```

### パケット実装例
```rust
packet_impl!(KeepAlivePacket, 0);
packet_impl!(LoginPacket, 1);
packet_impl!(ChatMessagePacket, 3);
packet_impl!(PlayerPositionPacket, 13);

// ChatMessagePacket実装
#[derive(Debug, Clone)]
pub struct ChatMessagePacket {
    pub message: String,
}

impl ChatMessagePacket {
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }
    
    pub fn with_message(message: String) -> Self {
        Self { message }
    }
}

impl Packet for ChatMessagePacket {
    fn get_id(&self) -> u8 { 3 }
    
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(self.get_id());
        // 文字列長書き込み
        let message_bytes = self.message.as_bytes();
        buf.extend_from_slice(&(message_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(message_bytes);
    }
    
    fn deserialize(&mut self, buf: &[u8]) -> Result<(), PacketError> {
        if buf.len() < 3 {
            return Err(PacketError::BufferTooSmall);
        }
        
        let len = u16::from_be_bytes([buf[1], buf[2]]) as usize;
        if buf.len() < 3 + len {
            return Err(PacketError::BufferTooSmall);
        }
        
        self.message = String::from_utf8(&buf[3..3+len])
            .map_err(|_| PacketError::InvalidData)?;
        Ok(())
    }
    
    fn handle(&self, connection: &mut NetworkConnection) {
        connection.handle_chat_message(&self.message);
    }
}
```

### ネットワークマネージャ
```rust
pub struct NetworkManager {
    packets: Vec<Box<dyn Packet>>,
    compression_enabled: bool,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            packets: Vec::new(),
            compression_enabled: false,
        }
    }
    
    pub fn handle_packet(&mut self, data: &[u8]) -> Result<(), PacketError> {
        if data.is_empty() {
            return Ok(());
        }
        
        let packet_id = data[0];
        let packet = create_packet(packet_id, &data[1..])?;
        
        // 圧縮パケット処理
        if packet_id == 255 {
            self.handle_compressed_packet(&data[1..])?;
        } else {
            packet.handle(self);
        }
        
        Ok(())
    }
    
    fn handle_compressed_packet(&mut self, data: &[u8]) -> Result<(), PacketError> {
        use flate2::read::ZlibDecoder;
        
        let mut cursor = std::io::Cursor::new(data);
        let compressed_size = cursor.read_i32::<LittleEndian>()? as usize;
        let uncompressed_size = cursor.read_i32::<LittleEndian>()? as usize;
        
        let mut compressed_data = vec![0u8; compressed_size];
        cursor.read_exact(&mut compressed_data)?;
        
        let mut decoder = ZlibDecoder::new(&compressed_data[..])?;
        let mut decompressed = vec![0u8; uncompressed_size];
        decoder.read_exact(&mut decompressed)?;
        
        self.handle_packet(&decompressed)
    }
    
    pub fn send_packet<P: Packet>(&mut self, packet: &P) -> Result<(), PacketError> {
        let mut buf = Vec::new();
        packet.serialize(&mut buf);
        
        // ネットワーク送信
        self.send_data(&buf)
    }
    
    fn send_data(&mut self, data: &[u8]) -> Result<(), PacketError> {
        // 実際のネットワーク送信処理
        println!("Sending {} bytes", data.len());
        Ok(())
    }
}
```

### オーディオシステム
```rust
pub struct AudioManager {
    device: rodio::Device,
    stream_handle: rodio::OutputStreamHandle,
    sounds: HashMap<String, rodio::Decoder>,
}

impl AudioManager {
    pub fn new() -> Result<Self, AudioError> {
        let (device, stream_handle) = rodio::OutputStream::try_default()?;
        
        Ok(Self {
            device,
            stream_handle,
            sounds: HashMap::new(),
        })
    }
    
    pub fn load_sound(&mut self, name: &str, path: &Path) -> Result<(), AudioError> {
        let file = std::fs::File::open(path)?;
        let decoder = rodio::Decoder::new(file)?;
        self.sounds.insert(name.to_string(), decoder);
        Ok(())
    }
    
    pub fn play_sound(&self, name: &str, position: Vec3<f32>, volume: f32) -> Result<(), AudioError> {
        if let Some(decoder) = self.sounds.get(name) {
            let sink = self.stream_handle.play_once(decoder.clone())?;
            sink.set_volume(volume);
            // 3Dポジション処理
            Ok(())
        } else {
            Err(AudioError::SoundNotFound(name.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum AudioError {
    DeviceError(rodio::PlayError),
    IoError(std::io::Error),
    SoundNotFound(String),
}
```

このネットワークシステムはMinecraftのマルチプレイヤー機能の中核であり、効率的なパケット処理と安定した通信が重要です。
