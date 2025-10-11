import { defaultPlugins, defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
    input: 'http://127.0.0.1:8080/api/openapi.json',
    output: {
        format: 'prettier',
        path: 'src/client',
    },
    plugins: [
        ...defaultPlugins,
        {
            name: '@hey-api/client-fetch',
            baseUrl: '',
        },
        '@tanstack/react-query',
    ],
});
