import { type ReactStyle, getActionFormsOptions } from '@rustforweb/shield-react';
import { useQuery } from '@tanstack/react-query';

export type ActionProps = {
    style: ReactStyle;
    params: {
        actionId?: string;
    };
};

export const Action = ({ style: Style, params: { actionId = 'index' } }: ActionProps) => {
    const {
        isPending,
        data: actionForms,
        error,
    } = useQuery(
        getActionFormsOptions({
            path: {
                actionId,
            },
        }),
    );

    // TODO: Use suspense query.
    if (isPending) {
        return 'Loading...';
    }

    if (error) {
        throw error;
    }

    return <Style action={actionForms} />;
};
