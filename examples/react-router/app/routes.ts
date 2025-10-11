import { type RouteConfig, index, prefix, route } from '@react-router/dev/routes';

export default [
    index('routes/home.tsx'),

    ...prefix('auth', [
        index('routes/auth/action.tsx', {
            id: 'routes/auth/action-index',
        }),
        route(':actionId', 'routes/auth/action.tsx'),
    ]),
] satisfies RouteConfig;
