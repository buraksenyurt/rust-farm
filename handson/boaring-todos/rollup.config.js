import rollup_nre from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

export default [
  {
    input: './src/main.js',
    output: {
      file: './dist/js/app-bundle.js',
      format: 'iife',
      name: 'bundle',
      sourcemap: true
    },
    plugins: [
      rollup_nre(),
      commonjs(),
    ]
  }
]