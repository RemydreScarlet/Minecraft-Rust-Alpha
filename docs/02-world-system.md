# Minecraft Alpha 1.1.2_01 - ワールドシステムドキュメント

## 1. ワールドアーキテクチャ (cn.java)

### クラス概要
`cn` - すべてのゲーム状態、チャンク、エンティティ、ワールドデータを管理するメインワールドクラス。

### コアデータ構造

#### ワールドプロパティ
```java
public class cn implements nm {
    // チャンク管理
    private List z;        // チャンクプロバイダー
    public List a;         // 読み込まれたチャンク
    private List A;        // アンロード中のチャンク
    private TreeSet B;      // チャンクアンロードキュー
    private Set C;         // チャンク読み込みセット
    
    // エンティティ管理
    public List b;         // ワールド内のすべてのエンティティ
    public List k;         // プレイヤーエンティティ
    
    // ワールドプロパティ
    public long c;         // ワールド時間
    public boolean d;       // 雪が覆われているフラグ
    public int e;          // ディメンション (0=オーバーワールド)
    protected int f, g;    // ワールドシード
    public boolean h;       // マルチプレイヤーフラグ
    public Random n;       // 乱数生成器
    
    // スポーンと位置
    public int o, p, q;   // スポーン座標 (X,Y,Z)
    public boolean r;       // 新しいワールドフラグ
    public File t;         // ワールドセーブディレクトリ
    public long u;         // ワールドシード
    public long v;         // ディスク上のサイズ
    public final String w;  // ワールド名
    public boolean x;       // 生成中フラグ
    public boolean y;       // デバッグフラグ
}
```

### ワールド初期化

#### コンストラクタフロー
```java
public cn(File var1, String var2, long var3) {
    // データ構造を初期化
    this.z = new ArrayList();     // チャンクプロバイダー
    this.a = new ArrayList();     // Loaded chunks
    this.b = new ArrayList();     // Entities
    
    // Set world properties
    this.w = var2;              // World name
    this.u = var3;              // World seed
    this.t = new File(var1, var2); // Save directory
    
    // Create session lock
    File sessionLock = new File(this.t, "session.lock");
    // Write unique timestamp to prevent concurrent access
    
    // Load existing world or create new
    if(levelDat.exists()) {
        loadWorldData();  // Load from NBT format
    } else {
        generateNewWorld(); // Create fresh world
    }
    
    // Initialize chunk provider
    this.H = this.a(this.t); // Creates ft (chunk provider)
}
```

#### World Loading Process
1. **Session Lock**: Create unique timestamp lock file
2. **Level Data**: Load `level.dat` (NBT format) containing:
   - RandomSeed: World generation seed
   - SpawnX/Y/Z: Player spawn position
   - Time: World time in ticks
   - SizeOnDisk: Estimated save file size
   - SnowCovered: Winter mode flag
   - Player: Player NBT data

3. **Chunk Provider**: Initialize `ft` (FileChunkProvider) for chunk loading/saving

## 2. Chunk System (ga.java)

### Class Overview
`ga` - Represents a 16x16x128 chunk of blocks and associated data.

### Chunk Data Structure

#### Block Storage
```java
public class ga {
    // Core block data
    public byte[] b;        // Block types (16*16*128 = 32768 bytes)
    public byte[] h;        // Height map (16*16 = 256 bytes)
    
    // Metadata storage (compressed)
    public mu e, f, g;     // Nibble arrays for block metadata
    
    // Chunk properties
    public final int j, k;  // Chunk coordinates (X, Z)
    public cn d;            // World reference
    public boolean c;        // Terrain populated flag
    public boolean n;        // Dirty flag (needs saving)
    public boolean o;        // Light calculated flag
    public boolean p;        // Loaded flag
    public boolean q;        // Queued for unload
    public boolean r;        // Modified flag
    
    // Entity storage
    public List[] m;        // Entity lists by Y-section (8 sections)
    public Map l;           // Tile entities by position
    
    // Height and optimization
    public int i;           // Minimum height in chunk
    public long s;          // Last save timestamp
}
```

#### Block Indexing
```java
// 3D to 1D array indexing
public int a(int x, int y, int z) {
    return x << 11 | z << 7 | y;  // (x * 2048) + (z * 128) + y
}

// Height map indexing
public int b(int x, int z) {
    return h[z << 4 | x] & 255;  // (z * 16) + x
}
```

**Indexing Logic**:
- X coordinate: 11 bits (0-15, shifted left 11 = *2048)
- Z coordinate: 7 bits (0-15, shifted left 7 = *128)  
- Y coordinate: 7 bits (0-127, no shift)
- Total: 32768 blocks per chunk (16×16×128)

### Chunk Operations

#### Block Placement
```java
public boolean a(int x, int y, int z, int blockId, int metadata) {
    int currentBlock = this.a(x, y, z);
    int currentMeta = this.e.a(x, y, z);
    
    if (currentBlock == blockId && currentMeta == metadata) {
        return false; // No change needed
    }
    
    // Update block data
    int worldX = this.j * 16 + x;
    int worldZ = this.k * 16 + z;
    this.b[x << 11 | z << 7 | y] = (byte)blockId;
    this.e.a(x, y, z, metadata);
    
    // Handle block removal
    if (currentBlock != 0) {
        ly.n[currentBlock].b(this.d, worldX, y, worldZ);
    }
    
    // Handle block placement
    if (blockId != 0) {
        // Update height map if solid block
        if (ly.r[blockId] != 0) {
            if (y >= currentHeight) {
                this.g(x, y + 1, z); // Update height above
            }
        }
        
        // Call block on-place handler
        ly.n[blockId].e(this.d, worldX, y, worldZ);
    }
    
    // Mark chunk as modified
    this.o = true; // Needs light recalculation
    this.n = true; // Needs saving
    
    return true;
}
```

#### Height Map Calculation
```java
public void b() { // Calculate height map
    int minHeight = 127;
    
    for (int x = 0; x < 16; ++x) {
        for (int z = 0; z < 16; ++z) {
            int height = 127;
            
            // Find highest non-air block
            int index = x << 11 | z << 7;
            while (height > 0 && ly.r[this.b[index + height - 1]] == 0) {
                --height;
            }
            
            this.h[z << 4 | x] = (byte)height;
            if (height < minHeight) {
                minHeight = height;
            }
        }
    }
    
    this.i = minHeight; // Store minimum height
    this.o = true;    // Mark as modified
}
```

### Light System

#### Light Calculation
```java
private void g(int x, int y, int z) {
    int currentHeight = this.h[z << 4 | x] & 255;
    int newHeight = currentHeight;
    
    if (y > currentHeight) {
        newHeight = y;
    }
    
    // Propagate light downward until hitting solid block
    int index = x << 11 | z << 7;
    while (newHeight > 0 && ly.r[this.b[index + newHeight - 1]] == 0) {
        --newHeight;
    }
    
    if (newHeight != currentHeight) {
        // Update light levels for changed column
        this.d.f(x, z, newHeight, currentHeight);
        this.h[z << 4 | x] = (byte)newHeight;
        
        // Update minimum chunk height
        if (newHeight < this.i) {
            this.i = newHeight;
        }
    }
}
```

### Entity Management

#### Entity Storage
```java
public void a(kh entity) { // Add entity to chunk
    if (!this.q) { // Not unloading
        this.r = true; // Mark as having entities
        
        // Verify entity is in correct chunk
        int chunkX = eo.b(entity.ak / 16.0D);
        int chunkZ = eo.b(entity.am / 16.0D);
        
        if (chunkX != this.j || chunkZ != this.k) {
            System.out.println("Wrong location! " + entity);
        }
        
        // Determine Y-section based on entity Y position
        int section = eo.b(entity.al / 16.0D);
        section = Math.max(0, Math.min(section, this.m.length - 1));
        
        entity.aZ = true; // Mark as in chunk
        entity.ba = this.j;  // Store chunk coordinates
        entity.bb = section; // Store Y-section
        entity.bc = this.k;
        
        this.m[section].add(entity); // Add to appropriate section
    }
}
```

#### Spatial Partitioning
- **Y-Sections**: 8 sections of 16 blocks each (0-127)
- **Purpose**: Optimizes entity collision detection
- **Query**: Only check entities in relevant Y-sections

### Serialization

#### Chunk Data Format
```java
public int a(byte[] data, int xOffset, int zOffset, int yStart, int yEnd, int dataIndex) {
    // Copy block data
    for (int x = xOffset; x < xEnd; ++x) {
        for (int z = zOffset; z < zEnd; ++z) {
            int index = x << 11 | z << 7 | yStart;
            int length = yEnd - yStart;
            System.arraycopy(data, dataIndex, this.b, index, length);
            dataIndex += length;
        }
    }
    
    // Copy metadata (nibble arrays)
    // Copy light data (nibble arrays)
    // Copy sky light data (nibble arrays)
    
    return dataIndex;
}
```

## 3. World Interface (nm.java)

### Core Operations
```java
public interface nm {
    // Block access
    int a(int x, int y, int z);           // Get block type
    boolean a(int x, int y, int z, int blockId, int metadata); // Set block
    boolean d(int x, int y, int z);        // Is chunk loaded?
    
    // Chunk access
    ga a(int chunkX, int chunkZ);          // Get chunk
    ga b(int chunkX, int chunkZ);          // Load/get chunk
    
    // Entity access
    void a(kh entity);                    // Add entity
    void b(kh entity);                    // Remove entity
    
    // World properties
    boolean g(int x, int y, int z);        // Is solid block?
    boolean a(int x, int y, int z, int facing); // Can place block?
}
```

## 4. Coordinate Systems

### World Coordinates
- **Range**: ±32,000,000 blocks (hard limit)
- **Type**: Integer coordinates for block positions
- **Origin**: (0, 0) at world center

### Chunk Coordinates  
- **Conversion**: `chunkX = worldX >> 4` (divide by 16)
- **Range**: ±2,000,000 chunks
- **Storage**: 16×16×128 blocks per chunk

### Local Coordinates
- **Range**: 0-15 within chunk
- **Indexing**: `localX = worldX & 15` (mod 16)
- **Purpose**: Efficient array indexing

## 5. Performance Optimizations

### Chunk Loading Strategy
1. **Circular Loading**: Load chunks around player in spiral pattern
2. **Distance Culling**: Only load chunks within render distance
3. **Priority Queue**: Prioritize chunks closer to player
4. **Background Loading**: Load chunks in separate thread

### Memory Management
1. **Chunk Pooling**: Reuse chunk objects to reduce GC
2. **Lazy Loading**: Only generate chunks when accessed
3. **Compression**: Metadata stored in nibble (4-bit) arrays
4. **Height Maps**: Pre-calculated for collision detection

### Rendering Optimizations
1. **Frustum Culling**: Only render visible chunks
2. **Occlusion Queries**: Hardware visibility testing
3. **Display Lists**: Pre-compiled chunk geometry
4. **Vertex Buffering**: Batch similar block types

## Rust Implementation Considerations

### Data Structures
```rust
// Block storage (32768 bytes per chunk)
struct Chunk {
    blocks: [u8; 32768],      // Block types
    metadata: [u8; 16384],     // 4 bits per block
    light: [u8; 16384],        // Block light
    sky_light: [u8; 16384],    // Sky light
    height_map: [u8; 256],     // Height map
    position: ChunkPos,         // Chunk coordinates
    dirty: bool,                // Needs saving
    modified: bool,             // Needs rebuild
}

// World coordinate
struct WorldPos {
    x: i32,
    y: i32, 
    z: i32,
}

// Chunk coordinate
struct ChunkPos {
    x: i32,
    z: i32,
}
```

### Performance Optimizations
1. **Memory Layout**: Use struct-of-arrays for better cache locality
2. **Indexing**: Pre-calculate array offsets for fast access
3. **Compression**: Use bit manipulation for metadata storage
4. **Parallel Processing**: Use rayon for parallel chunk operations

### Safety Considerations
1. **Bounds Checking**: Validate all coordinate access
2. **Concurrency**: Use RwLock for thread-safe chunk access
3. **Memory Safety**: Avoid raw pointers, use safe indexing
4. **Error Handling**: Use Result types for fallible operations
