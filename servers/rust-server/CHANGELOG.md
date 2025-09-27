# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### ⛰️  Features

- Increased bots starting cash to $10,000 - ([cde2e6d](https://github.com/coding-kelps/finwar/commit/cde2e6de1dad6e94865512c3d068953507561857))
- A begining of data sanitazition on trade routes - ([6ca73af](https://github.com/coding-kelps/finwar/commit/6ca73aff8708df1c094695904037ecf9b63574ba))
- An enroll route and some code orga - ([06d30e9](https://github.com/coding-kelps/finwar/commit/06d30e9311dac521bb481552668c5cf6fa58eb9b))
- Sea-orm set up - ([ba5062a](https://github.com/coding-kelps/finwar/commit/ba5062a471b4333043e4970689922cf4314b073b))
- A docker-compose for the db - ([46286fa](https://github.com/coding-kelps/finwar/commit/46286fab4e2743932bc757e85ae6759f78ac1d32))
- A first orm db connection using sea-orm - ([456651b](https://github.com/coding-kelps/finwar/commit/456651bb92f58c7656add98a657009487c5d13c7))
- Handling app state to avoid duplicated csv loading / reading - ([b5cfa6e](https://github.com/coding-kelps/finwar/commit/b5cfa6e9ecd28e02a6939db4feda9c9d48459025))
- A draft of an historgram. not sure what we're looking at. - ([eca12d0](https://github.com/coding-kelps/finwar/commit/eca12d094725d73e74c660c38031c8b17ae59816))
- Distribution pie chart and a description - ([ce533a8](https://github.com/coding-kelps/finwar/commit/ce533a82ccba4cbc6331e47fa9ef37a7ea44cbe0))
- Separated files, basic templates and a first draft of a home page - ([75a5f5a](https://github.com/coding-kelps/finwar/commit/75a5f5aae0d44d0c2635dcf40ef9f574626af36e))
- Tracing and errors - ([391a741](https://github.com/coding-kelps/finwar/commit/391a741e0fc44fed254a60004800fe535080889f))
- A simple candlestick plot using polars and plotly - ([af917cf](https://github.com/coding-kelps/finwar/commit/af917cf76569c890cbf0add91994114111312c8a))
- Handling app state to avoid duplicated csv loading / reading - ([edc2061](https://github.com/coding-kelps/finwar/commit/edc2061844068614eaf0a30fa152990fe774b56a))
- A draft of an historgram. not sure what we're looking at. - ([a5c5dca](https://github.com/coding-kelps/finwar/commit/a5c5dcac1203e5be7deef300041ad24835952cb6))
- Distribution pie chart and a description - ([42ff676](https://github.com/coding-kelps/finwar/commit/42ff676ae80b1ad506c20869e810b9fea3471575))
- Separated files, basic templates and a first draft of a home page - ([03e9592](https://github.com/coding-kelps/finwar/commit/03e959296f459eb0a90d8b94b603c1e96e372be7))
- Tracing and errors - ([16358ca](https://github.com/coding-kelps/finwar/commit/16358ca3f21451d268e15d22f81d22550ef82d94))
- A simple candlestick plot using polars and plotly - ([7cb260b](https://github.com/coding-kelps/finwar/commit/7cb260bda824105c65135b1ef9848d45cd5e2d77))
- Simple axum server (#3) - ([a6d0834](https://github.com/coding-kelps/finwar/commit/a6d083449e6c70829a48a066af1f8f4b6a3c3de8))
- Simple axum server (#3) - ([606b8ed](https://github.com/coding-kelps/finwar/commit/606b8edafa34624ddee5ae46c239687cac28530f))

### 🐛 Bug Fixes

- Unsufficient wallet cash numerical precision - ([0876ef6](https://github.com/coding-kelps/finwar/commit/0876ef6ef1fc30698f68848486d87741c86595d6))
- Uuid default values generated in pgsql - ([ad5a0ce](https://github.com/coding-kelps/finwar/commit/ad5a0ce8b4371e2faaa63d5cfd92bc87fd1c5ce2))
- Broken link to leaderboard - ([ca775b2](https://github.com/coding-kelps/finwar/commit/ca775b2d3534622f9d877e1f8745931ede16de4c))

### 🚜 Refactor

- Revoming .clone to prevent useless data dup - ([01d3700](https://github.com/coding-kelps/finwar/commit/01d37005cd8642798b2d9d2bce21381bffa843dd))
- Changing ordering of text - ([f1507c4](https://github.com/coding-kelps/finwar/commit/f1507c40d487540ef33a94816d07c944b79aebf2))
- Revoming .clone to prevent useless data dup - ([aeaabd8](https://github.com/coding-kelps/finwar/commit/aeaabd8445f012c993d2e828457d48b4f54ee141))
- Changing ordering of text - ([21023db](https://github.com/coding-kelps/finwar/commit/21023dba846ee80a3fa2e1cf97b7d3370d3601ae))

### 📚 Documentation

- Adding how to generate entities - ([f69eb12](https://github.com/coding-kelps/finwar/commit/f69eb1245e7cbf2409ac82e52d48f2b700331c32))
- A needed README with important data on how to run - ([ddc6436](https://github.com/coding-kelps/finwar/commit/ddc6436c140be0a8dc1a2a340aa8a748dc2f5beb))

### 🎨 Styling

- Moving unstable_features prop to the top of the file - ([d9298ed](https://github.com/coding-kelps/finwar/commit/d9298ed03687a4b4f5305a69fc7a38c31d26253b))
- Reordering mods and uses - ([a7f4f31](https://github.com/coding-kelps/finwar/commit/a7f4f3107f55b97cce4e04ed0754a83c0af16a96))
- Sharing a basic rustfmt conf - ([3d805f6](https://github.com/coding-kelps/finwar/commit/3d805f677439e1a1906a4fb3dff414c905a27fb6))
- Sharing a basic rustfmt conf - ([665d6e8](https://github.com/coding-kelps/finwar/commit/665d6e85e0dff2d89f22824f76c3a37279c2bbad))

### 🔨 Build

- Ignoring .env info - ([92c652d](https://github.com/coding-kelps/finwar/commit/92c652d5c4f5aef436ac4c80053a4a7251fd6b7a))
- Ignoring local folder - ([9e2d5b2](https://github.com/coding-kelps/finwar/commit/9e2d5b2d1b4dbcff12d0b7decdf4aac1ba97dfd7))
- Ignoring local folder - ([2fbfaa9](https://github.com/coding-kelps/finwar/commit/2fbfaa9ab5b316097cf123d2b871bc32d6f089c8))

### ⚙️ Miscellaneous Tasks

- Unify database docker compose - ([12d6683](https://github.com/coding-kelps/finwar/commit/12d66833e1b351ca84344d219ba427d655fe552b))


<!-- generated by git-cliff -->
