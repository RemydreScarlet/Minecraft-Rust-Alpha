# Minecraft Alpha 1.1.2_01 - Core Infrastructure Documentation

## 1. Game Loop Architecture (Minecraft.java)

### Class Overview
`net.minecraft.client.Minecraft` - The main game class that orchestrates all game systems.

### Key Components

#### Game Loop Structure
```java
// Main game loop (lines 359-475)
public void run() {
    this.F = true; // Game running flag
    
    // Initialization
    this.a(); // Initialize display and OpenGL
    
    // Main loop
    while(this.F && (this.z == null || this.z.isActive())) {
        cf.a(); // Timer update
        aj.a(); // System updates
        
        // Input handling
        if(this.k == null && Display.isCloseRequested()) {
            this.d(); // Shutdown
        }
        
        // Game tick processing
        for(int var17 = 0; var17 < this.M.b; ++var17) {
            this.i(); // Process game tick
        }
        
        // Rendering
        this.c("Pre render");
        this.A.a(this.g, this.M.c); // Sound system update
        // ... rendering pipeline
        Display.update();
    }
}
```

#### Timing System
- **Timer Class**: `ir` - Manages game tick timing (20 ticks per second)
- **Partial Ticks**: `this.M.c` - Float value for smooth interpolation between ticks
- **FPS Counter**: Tracks and displays frames per second

#### Display Management
- **Fullscreen Toggle**: Method `h()` handles fullscreen/windowed switching
- **Resolution Management**: Dynamic resolution changes with viewport updates
- **Canvas Integration**: Supports both applet and standalone modes

### State Management
- **Game States**: Controlled by `bh` (Screen) instances
- **World Loading**: Method `a(cn var1)` handles world transitions
- **Pause/Focus**: Manages game pause when window loses focus

## 2. Coordinate System and Math Utilities

### Position Class (a.java)
```java
public class a {
    public final int a; // X coordinate
    public final int b; // Y coordinate  
    public final int c; // Z coordinate
    public final int d; // Packed coordinate (X | Y<<10 | Z<<20)
}
```

**Purpose**: Immutable 3D integer coordinate for block positions
**Packing**: Uses bit manipulation for efficient storage and hashing
- X: 10 bits (0-1023)
- Y: 10 bits (0-1023) 
- Z: 10 bits (0-1023)

**Key Methods**:
- `a(a var1)`: Distance calculation between positions
- `equals(Object var1)`: Equality based on packed coordinate
- `hashCode()`: Returns packed coordinate for HashMap usage

### Math Utilities (eo.java)

#### Trigonometry Optimization
```java
private static float[] a = new float[65536]; // Sin lookup table

// Fast sine using lookup table
public static final float a(float var0) {
    return a[(int)(var0 * 10430.378F) & '\uffff'];
}

// Fast cosine using offset lookup
public static final float b(float var0) {
    return a[(int)(var0 * 10430.378F + 16384.0F) & '\uffff'];
}
```

**Optimization Strategy**: Pre-computed sine table with 65536 entries
- Index calculation: `angle * 10430.378F` maps to table indices
- Bit masking `& '\uffff'` handles wraparound
- Cosine uses sine table with π/2 offset

#### Utility Functions
- `c(float var0)`: Fast square root
- `d(float var0)`: Floor function with proper negative handling
- `e(float var0)`: Absolute value
- `a(double var0, double var2)`: Maximum of two values
- `a(int var0, int var1)`: Integer division with proper negative handling

## 3. OpenGL Utilities (df.java)

### Resource Management
```java
public class df {
    private static List a = new ArrayList(); // Display lists tracking
    private static List b = new ArrayList(); // Textures tracking
}
```

**Purpose**: Centralized OpenGL resource management and cleanup

#### Display List Management
```java
public static synchronized int a(int var0) {
    int var1 = GL11.glGenLists(var0);
    a.add(Integer.valueOf(var1));
    a.add(Integer.valueOf(var0));
    return var1;
}
```

#### Texture Management
```java
public static synchronized void a(IntBuffer var0) {
    GL11.glGenTextures(var0);
    // Track all generated texture IDs for cleanup
}
```

#### Buffer Management
```java
public static synchronized ByteBuffer b(int var0) {
    ByteBuffer var1 = ByteBuffer.allocateDirect(var0).order(ByteOrder.nativeOrder());
    return var1;
}

public static IntBuffer c(int var0) {
    return b(var0 << 2).asIntBuffer(); // Int buffer = 4 bytes per int
}

public static FloatBuffer d(int var0) {
    return b(var0 << 2).asFloatBuffer(); // Float buffer = 4 bytes per float
}
```

**Key Features**:
- Direct buffer allocation for optimal OpenGL performance
- Native byte order for cross-platform compatibility
- Automatic resource cleanup on shutdown

## 4. Input System Architecture

### Input Processing Pipeline
1. **Event Polling**: `Mouse.next()` and `Keyboard.next()` in main loop
2. **State Management**: Current and previous input states tracked
3. **Event Dispatch**: Sent to either game world or GUI based on current screen
4. **Binding System**: Key bindings stored in player settings

### Mouse Input
- **Button Handling**: Left click (break), Right click (place/use), Middle click (pick block)
- **Wheel Scrolling**: Hotbar slot selection
- **Movement**: Camera rotation when mouse is grabbed

### Keyboard Input
- **Movement Keys**: WASD for movement, Space for jump, Shift for sneak
- **Inventory Keys**: 1-9 for hotbar, E for inventory
- **System Keys**: F11 for fullscreen, ESC for pause menu

## 5. Memory Management

### Object Pooling
- **Display Lists**: Reused for chunk rendering
- **Buffers**: Pooled to reduce garbage collection
- **Entities**: Object pools for frequently spawned entities

### Garbage Collection Optimization
- **Direct Buffers**: Used for OpenGL data to avoid GC pressure
- **Immutable Objects**: Position and coordinate objects are immutable
- **Pre-allocation**: Large arrays pre-allocated to avoid runtime allocation

## 6. Performance Considerations

### Rendering Optimizations
- **Frustum Culling**: Only render visible chunks
- **Occlusion Queries**: Hardware-accelerated visibility testing
- **Display Lists**: Pre-compiled geometry for static blocks

### Threading
- **Main Thread**: Game logic and rendering
- **Chunk Loading**: Separate thread for world generation
- **Resource Loading**: Background loading of textures and sounds

### Memory Layout
- **Chunk Storage**: 16x16x128 block arrays
- **Block Data**: Single byte per block (type + metadata)
- **Entity Lists**: Spatially partitioned for efficient updates

## Rust Implementation Notes

### Data Structures
- Use `Vec<i8>` for block data with typed wrappers
- Implement coordinate struct with `#[repr(C)]` for packed representation
- Use `Arc<Mutex<>>` for thread-safe resource sharing

### Performance
- Implement lookup tables for trigonometric functions
- Use memory pools for frequently allocated objects
- Leverage Rust's zero-cost abstractions for math operations

### Safety
- Replace raw OpenGL calls with safe wrappers
- Use RAII for resource management
- Implement proper error handling for OpenGL operations
