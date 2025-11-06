# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0](https://github.com/coding-kelps/finwar/compare/market-api-v0.1.0..market-api-v0.2.0) - 2025-11-06

### ⛰️  Features

- [**breaking**] Bump project version - ([575238c](https://github.com/coding-kelps/finwar/commit/575238caadc75a855d8d05a0dc4c39e6f1aa37b4))
- Implement CLI for migrate and serve command - ([fb6f5ab](https://github.com/coding-kelps/finwar/commit/fb6f5ab1012c0a8ee0d0bc63667643ad303f6522))

### ⚙️ Miscellaneous Tasks

- Generalized release ci - ([2901a96](https://github.com/coding-kelps/finwar/commit/2901a9677609137b4ecd7b616900aca10c344243))


## [0.1.0] - 2025-11-03

### ⛰️  Features

- Load more button for orderbook table - ([ea3aa15](https://github.com/coding-kelps/finwar/commit/ea3aa158c91cbbbcc3fa34a5065fcb35e830bee1))
- Generic css style for buttons - ([0bfaa44](https://github.com/coding-kelps/finwar/commit/0bfaa44812c2672ea9b40fb6f796e1e1e2e40617))
- Load more button for ranking table - ([6ef79ab](https://github.com/coding-kelps/finwar/commit/6ef79abba8c16d9c531ae3a4d4a0cf3f6e2b88fd))
- Buy and sell attempts - ([532add6](https://github.com/coding-kelps/finwar/commit/532add6fcc96f4de49e1829caa6adf1b51c091cf))
- Time and price are now displayed on the frontend - ([74dcc68](https://github.com/coding-kelps/finwar/commit/74dcc682ba0e5795b31f2ca0ea637afa997e0750))
- Internal MarketClock - ([fd1e220](https://github.com/coding-kelps/finwar/commit/fd1e220e8a2edf3a4fb984503a1d7ab50576000d))
- Integrate charming for chart rendering and update stocks history entity - ([d7dc41e](https://github.com/coding-kelps/finwar/commit/d7dc41e89025696443e6725ad48eec7897ea1f42))
- Enforce unique bot names and handle duplicate name errors - ([6f52310](https://github.com/coding-kelps/finwar/commit/6f52310726bb873593301654e515e267243a07e6))
- Enhance error page to display status code and improved message - ([50f6d74](https://github.com/coding-kelps/finwar/commit/50f6d74e7d804a1ec64bc9cb028eaada6d520ea3))
- Add order details to leaderboard and enhance bot information display - ([7d80df9](https://github.com/coding-kelps/finwar/commit/7d80df98bfb849e3a658928916c0ef4afcb58fb6))
- Update wallet calculations and format leaderboard cash/profit values - ([e525f12](https://github.com/coding-kelps/finwar/commit/e525f1289fae4ca28c510be15e7cda2db62a559c))
- Enhance bot detail page with order details and improve order enrollment logic - ([44cdd1c](https://github.com/coding-kelps/finwar/commit/44cdd1cf86c9bed84cc4c73ce9ab5be514dd3968))
- Add orderbook table migration - ([33c5a81](https://github.com/coding-kelps/finwar/commit/33c5a8159c2c4483a407774500e2d9a6fe63b3c5))
- Add profit metrics to leaderboard with starting cash/assets in state - ([e08d484](https://github.com/coding-kelps/finwar/commit/e08d4841f8509e9c28132f2bb21289a5183cff76))
- Add bot detail page and handler - ([73b74cb](https://github.com/coding-kelps/finwar/commit/73b74cb4109a94244e670133ae6dca3453fc5d37))
- Implement leaderboard ranking and bot detail route - ([0c4ad9a](https://github.com/coding-kelps/finwar/commit/0c4ad9a049cee7d0306508c74fb20a5f35fe55a9))
- Implement leaderboard page with bot count display - ([01c9775](https://github.com/coding-kelps/finwar/commit/01c97758ee27d3fd0d5783504bc704aa87f6b04d))
- Add static file serving for CSS assets - ([2384e09](https://github.com/coding-kelps/finwar/commit/2384e09cbc422ff749c4fc1777a2ca7ee81a172a))
- Add Dockerfile and docker-compose service for containerization - ([a6f649d](https://github.com/coding-kelps/finwar/commit/a6f649d3c4436eb97fb484f9b7c264112148e8b9))
- Register stocks_history migration - ([6a5aa2a](https://github.com/coding-kelps/finwar/commit/6a5aa2abef7fa97cf4e24e731c44e1fb277d1010))
- Add stocks_history table migration - ([23b1be7](https://github.com/coding-kelps/finwar/commit/23b1be76998a4e84228a62335c2d63498abde83a))
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

- Correct formatting of bot cash and assets description - ([62ca846](https://github.com/coding-kelps/finwar/commit/62ca8463ba6209786d65384c2bbec07a19a26ba4))
- Correct home link in leaderboard and update homepage title - ([a164738](https://github.com/coding-kelps/finwar/commit/a1647380a2bec8dcaa50e99cd34e2cd2c1be49ca))
- Include static assets in Docker image - ([c6b49ce](https://github.com/coding-kelps/finwar/commit/c6b49cedc63b4aaa1b3ef0e962b6a75af1d901d1))
- Comment out database migration calls in main function - ([056653e](https://github.com/coding-kelps/finwar/commit/056653e62cdc8a2010627d68a10b381526e56ae8))
- Replace plotly browser dependency with kaleido renderer - ([bfb02b9](https://github.com/coding-kelps/finwar/commit/bfb02b9bd96890b8c5133e2ba4c75b0494f0c28b))
- Unsufficient wallet cash numerical precision - ([0876ef6](https://github.com/coding-kelps/finwar/commit/0876ef6ef1fc30698f68848486d87741c86595d6))
- Uuid default values generated in pgsql - ([ad5a0ce](https://github.com/coding-kelps/finwar/commit/ad5a0ce8b4371e2faaa63d5cfd92bc87fd1c5ce2))
- Broken link to leaderboard - ([ca775b2](https://github.com/coding-kelps/finwar/commit/ca775b2d3534622f9d877e1f8745931ede16de4c))

### 🚜 Refactor

- Adding time to orderbook table, commenting stock - ([43d9338](https://github.com/coding-kelps/finwar/commit/43d9338c54d9633534a453ccb14927b2336fdb01))
- Functional order book - ([3eddb6f](https://github.com/coding-kelps/finwar/commit/3eddb6f66139c0db55991172bfb802e9b45e3633))
- Rollingback enroll to a normal usage - ([6101ea7](https://github.com/coding-kelps/finwar/commit/6101ea7a71f7fce83d4f60ac291495b3751eb14e))
- Small style home page improvements - ([545a72f](https://github.com/coding-kelps/finwar/commit/545a72f862c8a486d39a2fe6ce10c0720cc5beb6))
- Home template text and small bugfixes - ([37d8068](https://github.com/coding-kelps/finwar/commit/37d8068e0d659d5b55fb5337fcb3bbc3beee73e4))
- Remove Plotly script and clean up content - ([1748f2b](https://github.com/coding-kelps/finwar/commit/1748f2bcfd57794069322caa9560032b043b8915))
- Remove Plotly and Polars dependencies, clean up home template - ([8c6ba7b](https://github.com/coding-kelps/finwar/commit/8c6ba7b3f7473060f68e81d6b5dbd139b68a2d33))
- Default starting values are now in state - ([1374036](https://github.com/coding-kelps/finwar/commit/1374036b396cfcf63663e8df2ae36f1b52c7e64a))
- Revoming .clone to prevent useless data dup - ([01d3700](https://github.com/coding-kelps/finwar/commit/01d37005cd8642798b2d9d2bce21381bffa843dd))
- Changing ordering of text - ([f1507c4](https://github.com/coding-kelps/finwar/commit/f1507c40d487540ef33a94816d07c944b79aebf2))
- Revoming .clone to prevent useless data dup - ([aeaabd8](https://github.com/coding-kelps/finwar/commit/aeaabd8445f012c993d2e828457d48b4f54ee141))
- Changing ordering of text - ([21023db](https://github.com/coding-kelps/finwar/commit/21023dba846ee80a3fa2e1cf97b7d3370d3601ae))

### 📚 Documentation

- Correct JSON formatting in bot creation curl command - ([d26af4e](https://github.com/coding-kelps/finwar/commit/d26af4edc0d47d028188c87811bee43b8c829097))
- Update README with additional instructions for bot creation and formatting improvements - ([b6f9c07](https://github.com/coding-kelps/finwar/commit/b6f9c071d48a0dde52b5537e1109f6bb91b90299))
- Add Docker build and run instructions to README - ([fef11d1](https://github.com/coding-kelps/finwar/commit/fef11d15ac98eec5f049fddda0fe765cb96c467c))
- Adding how to generate entities - ([f69eb12](https://github.com/coding-kelps/finwar/commit/f69eb1245e7cbf2409ac82e52d48f2b700331c32))
- A needed README with important data on how to run - ([ddc6436](https://github.com/coding-kelps/finwar/commit/ddc6436c140be0a8dc1a2a340aa8a748dc2f5beb))
- Git cliff changelogs - ([4d28349](https://github.com/coding-kelps/finwar/commit/4d28349c8a7e24780c48917eff568f1cb6c13c8a))

### ⚡ Performance

- Optimize dependency compilation speed - ([0fecc88](https://github.com/coding-kelps/finwar/commit/0fecc880ebca0562fbe88a91e746c5dc7c90fc28))
- Optimize Dockerfile with cargo-chef and sccache - ([88442a9](https://github.com/coding-kelps/finwar/commit/88442a9d5817d867444b63929cef5497e3632891))

### 🎨 Styling

- Update bot page link and enhance CSS for strong elements - ([09f529b](https://github.com/coding-kelps/finwar/commit/09f529b5196c3826a3fce24453eed8b189ae6719))
- Tweak css and script placement in templates - ([fbfd8dc](https://github.com/coding-kelps/finwar/commit/fbfd8dcc928a802fb67d7cdbebc72f76fc8be35c))
- Fix static asset path and update leaderboard template - ([acb905d](https://github.com/coding-kelps/finwar/commit/acb905dec4af889f93765d6edadc1e0f07b0a20b))
- Update base and home templates with consistent styling - ([eddb6f2](https://github.com/coding-kelps/finwar/commit/eddb6f2324d3e24bf74b45bbec70952c8492887a))
- Moving unstable_features prop to the top of the file - ([d9298ed](https://github.com/coding-kelps/finwar/commit/d9298ed03687a4b4f5305a69fc7a38c31d26253b))
- Reordering mods and uses - ([a7f4f31](https://github.com/coding-kelps/finwar/commit/a7f4f3107f55b97cce4e04ed0754a83c0af16a96))
- Sharing a basic rustfmt conf - ([3d805f6](https://github.com/coding-kelps/finwar/commit/3d805f677439e1a1906a4fb3dff414c905a27fb6))
- Sharing a basic rustfmt conf - ([665d6e8](https://github.com/coding-kelps/finwar/commit/665d6e85e0dff2d89f22824f76c3a37279c2bbad))

### 🔨 Build

- Reduce container image size - ([4b20a48](https://github.com/coding-kelps/finwar/commit/4b20a48054e57bf221c4176d9379e2ec6920a123))
- Ignoring .env info - ([92c652d](https://github.com/coding-kelps/finwar/commit/92c652d5c4f5aef436ac4c80053a4a7251fd6b7a))
- Ignoring local folder - ([9e2d5b2](https://github.com/coding-kelps/finwar/commit/9e2d5b2d1b4dbcff12d0b7decdf4aac1ba97dfd7))
- Ignoring local folder - ([2fbfaa9](https://github.com/coding-kelps/finwar/commit/2fbfaa9ab5b316097cf123d2b871bc32d6f089c8))

### ⚙️ Miscellaneous Tasks

- Release ci - ([6ad3dcc](https://github.com/coding-kelps/finwar/commit/6ad3dcc253006d0545ce85db038320d267c7b89f))
- Unify database docker compose - ([12d6683](https://github.com/coding-kelps/finwar/commit/12d66833e1b351ca84344d219ba427d655fe552b))

## New Contributors ❤️

* @guilhem-sante made their first contribution
* @dfayd0 made their first contribution

<!-- generated by git-cliff -->
