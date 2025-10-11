import { Action } from '@rustforweb/shield-react-router';
import { ShadcnUiStyle } from '@rustforweb/shield-react-shadcn-ui';

import type { Route } from './+types/action';

const ActionRoute = (props: Route.ComponentProps) => {
    return <Action style={ShadcnUiStyle} {...props} />;
};

export default ActionRoute;
