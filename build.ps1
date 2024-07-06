wasm-pack build --target web

# del pkg\files\gamers_world_wasm_bg.wasm
# del pkg\files\gamers_world_wasm_bg.wasm.d.ts
# del pkg\files\gamers_world_wasm.js
# del pkg\files\gamers_world_wasm.d.ts
del pkg\.gitignore

# mv pkg/gamers_world_wasm_bg.wasm pkg/files
# mv pkg/gamers_world_wasm_bg.wasm.d.ts pkg/files
# mv pkg/gamers_world_wasm.js pkg/files
# mv pkg/gamers_world_wasm.d.ts pkg/files

echo "Build complete!"