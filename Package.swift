// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "Lettras",
    platforms: [.macOS(.v13), .iOS(.v16), .tvOS(.v16), .watchOS(.v9), .visionOS(.v1)],
    products: [.library(name: "Lettras", targets: ["Lettras"])],
    dependencies: [.package(url: "https://github.com/swiftwasm/WasmKit.git", exact: "0.2.2")],
    targets: [
        .target(
            name: "Lettras",
            dependencies: [.product(name: "WasmKit", package: "WasmKit")],
            path: "swift/Sources/Lettras",
            resources: [.copy("Resources/lettras_engine.wasm")]
        ),
        .testTarget(name: "LettrasTests", dependencies: ["Lettras"], path: "swift/Tests/LettrasTests"),
    ]
)
