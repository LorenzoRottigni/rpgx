import vue from 'rollup-plugin-vue';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';

export default {
  input: 'src/index.ts',
  output: [
    {
      file: 'dist/rpgx.cjs.js',
      format: 'cjs',
      exports: 'default'
    },
    {
      file: 'dist/rpgx.esm.js',
      format: 'esm'
    },
    {
      file: 'dist/rpgx.umd.js',
      format: 'umd',
      name: 'RPGX',
      globals: {
        vue: 'Vue'
      }
    }
  ],
  external: ['vue'],
  plugins: [
    resolve(),
    commonjs(),
    vue({ preprocessStyles: true }),
    typescript({ noCheck: true }),
    terser()
  ]
};
