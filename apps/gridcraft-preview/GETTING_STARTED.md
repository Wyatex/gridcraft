# GridCraft Preview - Getting Started

快速开始使用 GridCraft Preview 构建 XLSX/XLS 预览功能。

## 🚀 快速开始

### 前置要求

```bash
# 1. 安装 Rust 和 wasm32 目标
rustup target add wasm32-unknown-unknown

# 2. 安装 Trunk (WASM 构建工具)
cargo install trunk
```

### 本地开发测试

```bash
# 克隆仓库 (如果还没有)
git clone https://github.com/Wyatex/gridcraft.git
cd gridcraft

# 切换到预览功能分支
git checkout feature/gridcraft-preview

# 进入预览项目目录
cd apps/gridcraft-preview

# 启动开发服务器
trunk serve --release
```

打开浏览器访问 http://127.0.0.1:8772

现在可以：
- ✅ 拖拽 XLSX/XLS 文件到页面
- ✅ 使用文件选择器上传文件
- ✅ 测试缩放、切换工作表、搜索等功能
- ✅ 切换语言和主题

## 📦 构建 NPM 包

### 步骤 1: 构建 WASM 模块

```bash
cd apps/gridcraft-preview
trunk build --release
```

输出位置: `../../dist/preview/`

### 步骤 2: 打包 NPM

```bash
cd npm
node scripts/build.js
```

这会：
1. 检查必需工具 (trunk, wasm32 target)
2. 构建 WASM 模块
3. 复制文件到 `npm/dist/`
4. 生成入口文件
5. 复制许可证文件

### 步骤 3: 测试包

```bash
# 查看包内容
npm pack --dry-run

# 创建 .tgz 文件
npm pack

# 在其他项目中测试
npm install /path/to/gridcraft-preview-0.1.0.tgz
```

### 步骤 4: 发布到 NPM (可选)

```bash
# 登录 NPM
npm login

# 发布
npm publish

# 或发布为 beta 版本
npm publish --tag beta
```

## 💻 在项目中使用

### 基本 HTML 示例

```html
<!DOCTYPE html>
<html>
<head>
  <title>GridCraft Preview Demo</title>
  <style>
    #gridcraft_canvas { width: 100%; height: 600px; }
  </style>
</head>
<body>
  <input type="file" id="fileInput" accept=".xlsx,.xls" />
  <button onclick="preview.zoomIn()">放大</button>
  <button onclick="preview.zoomOut()">缩小</button>
  
  <canvas id="gridcraft_canvas"></canvas>
  <div id="gridcraft_loading">加载中...</div>

  <script type="module">
    import init, { GridCraftPreview } from './gridcraft-preview/index.js';

    // 初始化 WASM
    await init();
    window.preview = new GridCraftPreview();

    // 加载文件
    document.getElementById('fileInput').addEventListener('change', async (e) => {
      const file = e.target.files[0];
      if (file) {
        await preview.loadFile(file);
      }
    });
  </script>
</body>
</html>
```

### 从 URL 加载

```javascript
const response = await fetch('/api/files/spreadsheet.xlsx');
const blob = await response.blob();
await preview.loadFile(blob, 'spreadsheet.xlsx');
```

### 完整功能示例

查看 `npm/example.html` 获取完整的示例，包括：
- 文件上传和拖拽
- 所有控制按钮
- 语言和主题切换
- 错误处理
- 状态显示

## 🌍 多语言支持

```javascript
// 切换为中文
preview.setLanguage('zh');

// 切换为英文
preview.setLanguage('en');

// 支持的语言
const languages = [
  'en', 'zh', 'es', 'ja', 'ko', 'ru', 'pt', 
  'de', 'cs', 'fr', 'sr', 'tr', 'pl', 'it', 
  'uk', 'vi', 'nl', 'ro', 'el'
];
```

## 🎨 主题切换

```javascript
preview.setTheme('light');  // 浅色主题
preview.setTheme('dark');   // 深色主题
preview.setTheme('auto');   // 跟随系统
```

## 📚 文档导航

根据你的需求选择相应的文档：

### 我想...

#### → 在网页中使用预览功能
📖 阅读 [USAGE.md](./USAGE.md) - 完整的使用指南，包含 React/Vue 集成示例

#### → 构建和发布 NPM 包
📖 阅读 [DEPLOY.md](./DEPLOY.md) - 部署指南，包含 CI/CD 配置

#### → 了解架构和技术细节
📖 阅读 [README.md](./README.md) - 项目架构和技术实现

#### → 查看 NPM 包文档
📖 阅读 [npm/README.md](./npm/README.md) - NPM 包的使用说明

#### → 查看完整示例
📖 打开 [npm/example.html](./npm/example.html) - 功能完整的演示页面

## 🔧 API 快速参考

### 初始化

```javascript
import init, { GridCraftPreview } from 'gridcraft-preview';
await init();
const preview = new GridCraftPreview();
```

### 文件操作

```javascript
await preview.loadFile(file);               // 加载文件
```

### 视图控制

```javascript
preview.zoomIn();                           // 放大
preview.zoomOut();                          // 缩小
preview.zoomToFit();                        // 适应窗口
preview.setZoom(150);                       // 设置为 150%
```

### 工作表导航

```javascript
const sheets = preview.getSheetNames();     // 获取所有工作表名称
const current = preview.getCurrentSheet();  // 当前工作表
preview.switchSheet('Sheet2');              // 切换工作表
```

### 搜索

```javascript
const count = preview.find('total');        // 查找文本
preview.findNext();                         // 下一个
preview.findPrevious();                     // 上一个
```

### 选择

```javascript
const range = preview.getSelection();       // 获取选区
preview.selectRange('A1:C10');              // 选择范围
```

## ⚠️ 重要提示

### 功能限制

GridCraft Preview 是**只读**预览工具，以下功能**已禁用**：
- ❌ 编辑单元格内容
- ❌ 保存或导出文件
- ❌ 修改格式
- ❌ 插入/删除行列
- ❌ 编辑公式

### 当前状态

项目状态: **Alpha** - 核心功能完成，可用于测试

**已完成**:
- ✅ WASM 构建基础设施
- ✅ 只读模式命令过滤
- ✅ UI 配置
- ✅ NPM 打包
- ✅ TypeScript 类型定义
- ✅ 文档和示例

**待完善** (不影响基本使用):
- ⏳ JavaScript API 与运行中应用的完整连接
- ⏳ 异步文件加载的 inbox 实现
- ⏳ 事件系统 (onFileLoaded, onSheetChanged 等)
- ⏳ 更多 API 方法 (getCell, getCellValue 等)

## 🐛 故障排除

### "trunk not found"
```bash
cargo install trunk
```

### "wasm32-unknown-unknown target not installed"
```bash
rustup target add wasm32-unknown-unknown
```

### "Failed to initialize"
- 确保先调用 `await init()`
- 检查浏览器控制台错误
- 验证 WASM 文件正确加载

### 空白画布
- 确保 canvas 有明确的尺寸 (width/height)
- 检查 GridCraft Preview 是否成功加载
- 验证文件加载没有错误

## 📊 性能

- **WASM 大小**: ~3 MB (未压缩), ~1 MB (gzip 压缩)
- **支持的文件大小**: 可处理大型电子表格 (虚拟滚动)
- **浏览器要求**: Chrome 79+, Firefox 51+, Safari 15+

## 🔗 链接

- **GitHub**: https://github.com/Wyatex/gridcraft
- **分支**: `feature/gridcraft-preview`
- **上游**: https://github.com/storytold/gridcraft
- **Discord**: https://discord.gg/artcraft

## 🆘 获取帮助

- **GitHub Issues**: 在仓库中创建 issue
- **Discord**: 加入 ArtCraft 社区
- **文档**: 查看其他 `.md` 文件获取详细信息

## 📝 许可证

双重许可: MIT 或 Apache-2.0，任选其一。

---

**开始使用**: `trunk serve --release` 🚀

**需要帮助**: 查看 [USAGE.md](./USAGE.md) 📖

**准备发布**: 查看 [DEPLOY.md](./DEPLOY.md) 📦
