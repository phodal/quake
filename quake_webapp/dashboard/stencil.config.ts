import { Config } from '@stencil/core';

export const config: Config = {
  namespace: 'dashboard',
  buildEs5: false,
  outputTargets: [
    {
      type: 'dist',
      esmLoaderPath: '../loader',
    },
    {
      type: 'dist-custom-elements-bundle',
    },
    {
      type: 'docs-readme',
    },
    {
      // copy wasm
      type: 'dist',
      copy: [
        { src: '../node_modules/@quakeworks/quake_wasm/quake_wasm_bg.wasm', dest: 'quake_wasm_bg.wasm' }
      ]
    },
    {
      type: 'www',
      serviceWorker: null, // disable service workers
    },
  ],
};
