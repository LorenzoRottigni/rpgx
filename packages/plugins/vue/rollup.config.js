import vue from 'unplugin-vue/rollup';
import typescript from 'rollup-plugin-typescript2';
import wasm from '@rollup/plugin-wasm';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

export default {
  input: 'src/index.ts',
  output: {
    dir: 'dist',
    format: 'es',
    sourcemap: true,
  },
  plugins: [
    vue(),
    typescript({
      tsconfig: './tsconfig.json',
      clean: true,
      check: false,
    }),
    wasm(),
    resolve({
      extensions: ['.js', '.ts', '.vue', '.wasm'],
      browser: true,
    }),
    commonjs(),
  ],
};
