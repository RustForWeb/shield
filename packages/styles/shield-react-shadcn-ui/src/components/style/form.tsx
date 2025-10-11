import type { Form as ApiForm } from '@rustforweb/shield-react';

// import { type FieldValues, useForm } from 'react-hook-form';

import { FieldGroup } from '../ui/field.js';
import { StyleInput } from './input.js';

export type StyleFormProps = {
    actionId: string;
    methodId: string;
    providerId?: string | null;
    form: ApiForm;
};

export const StyleForm = ({ actionId, methodId, providerId, form: { inputs } }: StyleFormProps) => {
    // const form = useForm({
    //     defaultValues: inputs.reduce((prev, input) => {
    //         prev[input.name] = input.value ?? '';
    //         return prev;
    //     }, {} as FieldValues),
    // });

    // const handleSubmit = form.handleSubmit(async (data) => {
    //     // TODO
    //     console.log(data);

    //     await call({
    //         path: {
    //             actionId,
    //             methodId,
    //             providerId: providerId ?? null,
    //         },
    //         body: data,
    //     });
    // });

    return (
        // <form onSubmit={handleSubmit}>
        // TODO: Remove hardcoded URL
        <form action={`/api/auth/${methodId}/${actionId}/${providerId}`} method="post">
            <FieldGroup>
                {inputs.map((input) => (
                    // <StyleInput key={input.name} control={form.control} input={input} />
                    <StyleInput key={input.name} input={input} />
                ))}
            </FieldGroup>
        </form>
    );
};
