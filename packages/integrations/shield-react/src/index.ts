import type { ActionForms } from './client/types.gen.js';

export * from './client/index.js';

export type ReactStyleProps = {
    action: ActionForms;
};

export type ReactStyle = React.ComponentType<ReactStyleProps>;
